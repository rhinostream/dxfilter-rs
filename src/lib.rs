#![doc = include_str ! ("../README.md")]

use win_desktop_duplication::texture::Texture;
use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11DeviceContext4};

extern crate shader_macro;

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
pub use shader_macro::shader as compile_shader;


#[macro_use]
#[doc(hidden)]
pub mod shader_generator;

pub mod error;

pub use error::DxResult as Result;

pub mod shader;

pub mod color;

mod common_filters;

pub mod utils;

pub use common_filters::*;

/// Interface for interacting with various filters. Interface is defined so that you could create
/// Directx pipelines that involve multiple filters.
///
/// ## Example Usage:
/// ```
/// fn main () {
///     //...
///     let (device, context) = // acquire Device and DeviceContext
///
///     let input_tex = // create an input texture
///     let output_tex = // create an output texture
///
///     // create some directx filter. for example the following
///     let filter = ScaleARGBOrAYUV::new(&input_tex,&output_tex,&device);
///
///     // apply the filter
///     filter.apply_filter(&context);
///
///     // read from the output_texture
/// }
/// ```
pub trait DxFilter {
    /// takes directx device context and applies various vertex and pixel shaders to apply the filter.
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()>;

    /// configure the filter to use different input texture.
    fn set_input_tex(&mut self, tex: &Texture) -> Result<()>;

    /// configure the filter to use different output texture.
    fn set_output_tex(&mut self, tex: &Texture) -> Result<()>;
}