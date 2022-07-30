use win_desktop_duplication::texture::Texture;
use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11DeviceContext4};

/// Compile directx shader at compile time and returns byte code. it uses [`D3DCompile2`](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2).
/// any unexplained parameters are analogous to that function.
///
/// ## Syntax
///
/// ```
/// shader_compile!{
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
/// let data = shader_compile!{
///    src: "
///         int main() {
///             return 1;
///         }
///     ",
///     entry_point: "main"
///     target: "ps_5_0"
/// };
/// ```
pub use shader_macro::shader as shader_compile;

#[macro_use]
#[doc(hidden)]
pub mod shader_generator;

pub mod error;

pub use error::DxResult as Result;

pub mod shader;

pub mod color;

pub mod common_filters;

pub trait DxFilter {
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()>;
    fn set_input_tex(&mut self, tex: &Texture) -> Result<()>;
    fn set_output_tex(&mut self, tex: &Texture) -> Result<()>;
}