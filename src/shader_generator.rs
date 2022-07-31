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
#[cfg(doc)]
#[macro_export]
macro_rules! compile_shader {
    ()=>{
        // this implementation is a dummy one for docs.rs to build unimplemented!()
    }
}


/// Compile DirectX shader at compile time and generates a function. The function takes an
/// `ID3D11Device4` instance to produce respective shader.
///
/// ## Syntax:
///
/// ```
///  //                             ps for pixel shader       syntax for compile_shader! check
///  // name of func to create      vs for vertex shader      compile_shader docs for this syntax
///  //        |_________________        |      ____________________|
///  //                           \      |     /
///             generate_shader!(fn_name ps {...})
/// ```
///
/// ## Example Usage:
///
/// ```rust
///
///  use dx11_screencap::generate_shader;
///
///
///  // this directly translates to
///  // fn sample_pixel_shader(device: ID3D11Device4)->PixelShader { ... }
///  generate_shader!(sample_pixel_shader ps {
///     src: "
///         int main() {
///             return 1;
///         }
///     ",
///     entry_point: "main"
///     target: "ps_5_0"
///  });
///
///  fn main(){
///     // somehow acquire directx device
///     let sample_shader = sample_pixel_shader(device);
///  }
///
/// ```
///
#[macro_export]
macro_rules! generate_shader {
    ($name:ident ps $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::PixelShader> {
            #[cfg(not(doc))]
            {
                $crate::shader::PixelShader::new(&$crate::compile_shader! $content, device)
            }
            #[cfg(doc)]
            unimplemented!()
        }

    };
    ($name:ident vs $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::VertexShader> {
            #[cfg(not(doc))]
            {
                $crate::shader::VertexShader::new(&$crate::compile_shader! $content, device)
            }
            #[cfg(doc)]
            unimplemented!()
        }
    };
}

