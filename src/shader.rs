use windows::Win32::Graphics::Direct3D11::{ID3D11Device4, ID3D11PixelShader, ID3D11VertexShader};
use crate::Result;
use crate::error::DxCapError;

pub struct VertexShader(ID3D11VertexShader);

impl VertexShader {
    pub fn new(blob: &[u8], device: ID3D11Device4) -> Result<Self> {
        let shader = unsafe { device.CreateVertexShader(blob, None) };
        if let Err(e) = shader {
            return Err(DxCapError::Unknown(format!("{:?}", e)));
        }
        return Ok(Self(shader.unwrap()));
    }
    pub fn get_raw(&self) -> &ID3D11VertexShader {
        return &self.0;
    }
}

pub struct PixelShader(ID3D11PixelShader);

impl PixelShader {
    pub fn new(blob: &[u8], device: ID3D11Device4) -> Result<Self> {
        let shader = unsafe { device.CreatePixelShader(blob, None) };
        if let Err(e) = shader {
            return Err(DxCapError::Unknown(format!("{:?}", e)));
        }
        return Ok(Self(shader.unwrap()));
    }
    pub fn get_raw(&self) -> &ID3D11PixelShader {
        return &self.0;
    }
}
