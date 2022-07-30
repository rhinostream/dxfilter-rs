use win_desktop_duplication::texture::Texture;
use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11DeviceContext4};
pub use shader_macro::shader as shader_compile;

#[macro_use]
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