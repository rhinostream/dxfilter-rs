#![doc = include_str ! ("../README.md")]

use win_desktop_duplication::texture::Texture;
use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11DeviceContext4};

#[cfg(not(doc))]
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