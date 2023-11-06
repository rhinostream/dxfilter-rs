//! Contains utils for creating Vertex and Pixel shaders from compiled shader byte code.
//! you can easily use [`generate_shader!`] macro instead of these structs directly.
//!
use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11PixelShader, ID3D11VertexShader};
use crate::Result;
use crate::error::DxFilterErr;

/// structure to hold `ID3D11VertexShader`. Create new shader using [`generate_shader!`]
pub struct VertexShader(ID3D11VertexShader);

impl VertexShader {
    /// create a new VertexShader instance from compiled shader blob and directx device.
    pub fn new(blob: &[u8], device: ID3D11Device4) -> Result<Self> {
        let mut shader = None;
        if let Err(e) = unsafe { device.CreateVertexShader(blob, None, Some(&mut shader)) } {
            return Err(DxFilterErr::Unknown(format!("{:?}", e)));
        }
        return Ok(Self(shader.unwrap()));
    }

    /// get raw reference to `ID3D11VertexShader` instance
    pub fn as_raw_ref(&self) -> &ID3D11VertexShader {
        return &self.0;
    }
}

/// structure to hold `ID3D11PixelShader`. Create new instance using [`generate_shader!`]
pub struct PixelShader(ID3D11PixelShader);

impl PixelShader {
    /// create a new PixelShader instance from compiled shader blob and directx device.
    pub fn new(blob: &[u8], device: ID3D11Device4) -> Result<Self> {
        let mut shader = None;
        if let Err(e) = unsafe { device.CreatePixelShader(blob, None, Some(&mut shader)) } {
            return Err(DxFilterErr::Unknown(format!("{:?}", e)));
        }
        return Ok(Self(shader.unwrap()));
    }

    /// get raw reference to `ID3D11PixelShader` instance.
    pub fn as_raw_ref(&self) -> &ID3D11PixelShader {
        return &self.0;
    }
}
