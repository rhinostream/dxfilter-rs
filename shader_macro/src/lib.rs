extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::ffi::CString;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::ptr::null;
use std::slice::from_raw_parts;

use quote::quote;
use syn::{braced, bracketed, Ident, LitByteStr, LitInt, LitStr, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use windows::core::PCSTR;
use windows::Win32::Graphics::Direct3D::{D3D_SHADER_MACRO, ID3DBlob, ID3DInclude};
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile2;
use windows::Win32::Graphics::Hlsl::D3D_COMPILE_STANDARD_FILE_INCLUDE;

struct ShaderMacroInput {
    src_data: String,

    // this value is null terminated
    src_name: Option<CString>,

    // last entry should contain name and description as null pointers
    macros: Vec<ShaderMacro>,

    // marking it empty will put D3D_COMPILE_STANDARD_FILE_INCLUDE
    // include: &'static [ID3DInclude],

    // null_terminated string
    entry_point: CString,

    // null_terminated string
    target: CString,

    flags1: u32,

    flags2: u32,

    secondary_data_flags: u32,

    secondary_data: Vec<u8>,
}

impl Parse for ShaderMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut src_data: Option<String> = None;
        let mut src_name: Option<CString> = None;
        let mut macros: Vec<ShaderMacro> = Vec::new();
        let mut entry_point: Option<CString> = None;
        let mut target: Option<CString> = None;
        let mut flags1: u32 = 0;
        let mut flags2: u32 = 0;
        let mut secondary_data_flags: u32 = 0;
        let mut secondary_data: Vec<u8> = vec![];


        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let name = name.to_string();
            match name.as_str() {
                "src" => {
                    src_data = Some(input.parse::<LitStr>()?.value());
                }
                "src_file" => {
                    let file_name = input.parse::<LitStr>()?.value();
                    let abs_path = PathBuf::from(file_name);

                    src_data = Some(read_to_string(&abs_path).unwrap());
                    src_name = Some(CString::new(abs_path.parent().unwrap().to_str().unwrap()).unwrap());
                }
                "src_name" => {
                    src_name = Some(CString::new(input.parse::<LitStr>()?.value()).unwrap());
                }
                "defines" => {
                    let content;
                    let _ = braced!( content in input);
                    let macro_raw: Punctuated<ShaderMacro, Token![,]> = content.parse_terminated(ShaderMacro::parse, Comma)?;
                    for m in macro_raw {
                        macros.push(m)
                    }
                }
                "entry_point" => {
                    entry_point = Some(CString::new(input.parse::<LitStr>()?.value()).unwrap());
                }
                "target" => {
                    target = Some(CString::new(input.parse::<LitStr>()?.value()).unwrap());
                }
                "flags1" => {
                    flags1 = input.parse::<LitInt>()?.base10_parse::<u32>()?;
                }
                "flags2" => {
                    flags2 = input.parse::<LitInt>()?.base10_parse::<u32>()?;
                }
                "secondary_data_flags" => {
                    secondary_data_flags = input.parse::<LitInt>()?.base10_parse::<u32>()?;
                }
                "secondary_data" => {
                    secondary_data = input.parse::<LitByteStr>()?.value();
                }
                _ => panic!("unexpected input provided to shader")
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
                break;
            }
        }

        Ok(ShaderMacroInput {
            src_data: src_data.unwrap(),
            src_name,
            macros,
            entry_point: entry_point.unwrap(),
            target: target.unwrap(),
            flags1,
            flags2,
            secondary_data_flags,
            secondary_data,
        })
    }
}

struct ShaderMacro {
    name: CString,
    def: CString,
}

impl Parse for ShaderMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let name: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;
        let value: LitStr = content.parse()?;
        Ok(Self {
            name: CString::new(name.value()).unwrap(),
            def: CString::new(value.value()).unwrap(),
        })
    }
}


/// Compile directx shader at compile time and returns byte code. it uses [`D3DCompile2`](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2).
/// any unexplained parameters are analogous to that function.
///
/// ## Syntax
///
/// ```
/// compile_shader!{
///     src: "some shader source code"  // [required] either src or src_file is required
///     src_file: "path/to/shader/source/code" // [required] either src or src_file is required
///     entry_point: "main_func_name"   // [required] name of entry point function
///     target: "shader_profile"        // [required] shader used to compile the shader.
///     src_name: "path/to/shader/source/file"  // [optional] required for #include if any.
///                                             // This is auto generated when src_file is used.
///     defines: {                      // [optional] used to define shader macros before compiling
///         ("DEFINE_1","32"),
///     },
///     flags1: 0,                      // [optional] flags1
///     flags2: 0,                      // [optional] flags2
///     secondary_data_flags: 0         // [optional] secondary_data_flags
///     secondary_data: ""              // [optional] secondary_data
/// }
///
/// ```
///
/// ## Example usage
///
/// ```
/// let data = compile_shader!{
///    src: "
///         int main() {
///             return 1;
///         }
///     ",
///     entry_point: "main"
///     target: "ps_5_0"
/// };
/// ```
#[proc_macro]
pub fn shader(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as ShaderMacroInput);
    let input_name = PCSTR({
        if let Some(name) = &input.src_name {
            name.as_ptr()
        } else {
            null()
        }
    } as _);
    let secondary_data: *const u8 = {
        if input.secondary_data.len() > 0 {
            input.secondary_data.as_ptr()
        } else {
            null()
        }
    };

    let mut defines: Vec<D3D_SHADER_MACRO> = input.macros.iter().map(|v| {
        D3D_SHADER_MACRO {
            Name: PCSTR(v.name.as_ptr() as _),
            Definition: PCSTR(v.def.as_ptr() as _),
        }
    }).collect();
    defines.push(D3D_SHADER_MACRO {
        Name: PCSTR(null()),
        Definition: PCSTR(null()),
    });

    let mut error_msgs: Option<ID3DBlob> = None;
    let mut shader_bytes: Option<ID3DBlob> = None;


    let result = unsafe {
        D3DCompile2(
            input.src_data.as_ptr() as _,
            input.src_data.len(), input_name, Some(defines.as_ptr())
            , core::mem::transmute::<_, &ID3DInclude>(&(D3D_COMPILE_STANDARD_FILE_INCLUDE as u64)), PCSTR(input.entry_point.as_ptr() as _),
            PCSTR(input.target.as_ptr() as _), input.flags1, input.flags2, input.secondary_data_flags,
            Some(secondary_data as _),
            input.secondary_data.len(), &mut shader_bytes, Some(&mut error_msgs))
    };
    if result.is_err() {
        if let Some(error_msgs) = error_msgs {
            let bytes = unsafe { from_raw_parts(error_msgs.GetBufferPointer() as *const u8, error_msgs.GetBufferSize() as usize) };
            panic!("failed to compile shader. {}", std::str::from_utf8(bytes).unwrap());
        }
        result.unwrap();
    }

    if let Some(shader_bytes) = shader_bytes {
        let bytes = unsafe { from_raw_parts(shader_bytes.GetBufferPointer() as *const u8, shader_bytes.GetBufferSize() as usize) };
        return TokenStream::from(quote!(
        [#(#bytes),*]
        ));
    }
    panic!("compilation succeeded but no bytes were returned!");
}