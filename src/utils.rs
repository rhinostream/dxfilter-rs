//! various utilities for setup with directx. If you are using this in other applications, you
//! should not need these methods. However, for simple applications, these will be helpful

use std::ptr::null;
use win_desktop_duplication::texture::{ColorFormat, Texture, TextureDesc};
use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_FLAG, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_RESOURCE_MISC_FLAG, D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE, D3D11_USAGE_DEFAULT, D3D11CreateDevice, ID3D11Device4, ID3D11DeviceContext4};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_1};
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::core::Interface;
use crate::error::DxFilterErr;
use crate::Result;
pub use win_desktop_duplication::devices::{Adapter, AdapterFactory};
use core::default::Default;

/// create new input texture for filters
pub fn create_input_texture(device: &ID3D11Device4, tex_desc: TextureDesc, initial_data: Option<Vec<u8>>) -> Result<Texture> {
    return create_texture(device, tex_desc,
                          D3D11_USAGE_DEFAULT, D3D11_BIND_SHADER_RESOURCE,
                          Default::default(), initial_data);
}


/// create new output texture for filters
pub fn create_output_texture(device: &ID3D11Device4, tex_desc: TextureDesc, initial_data: Option<Vec<u8>>) -> Result<Texture> {
    return create_texture(device, tex_desc,
                          D3D11_USAGE_DEFAULT, D3D11_BIND_RENDER_TARGET,
                          Default::default(), initial_data);
}

/// create directx device and context from given adapter
pub fn create_device_context(adapter: &Adapter) -> Result<(ID3D11Device4, ID3D11DeviceContext4)> {
    let feature_levels = [D3D_FEATURE_LEVEL_11_1];
    let mut device = None;
    let mut ctx = None;
    let mut level = Default::default();
    let res = unsafe {
        D3D11CreateDevice(
            adapter.as_raw_ref(),
            D3D_DRIVER_TYPE_UNKNOWN,
            None, Default::default(),
            &feature_levels, D3D11_SDK_VERSION, &mut device, &mut level, &mut ctx)
    };

    if let Err(e) = res {
        return Err(DxFilterErr::Unknown(format!("failed to create Device. `{:?}`", e)));
    }
    let ctx: ID3D11DeviceContext4 = ctx.unwrap().cast().unwrap();
    let device: ID3D11Device4 = device.unwrap().cast().unwrap();

    return Ok((device, ctx));
}

fn create_texture(device: &ID3D11Device4, tex_desc: TextureDesc, usage: D3D11_USAGE, bind_flags: D3D11_BIND_FLAG,
                  misc_flag: D3D11_RESOURCE_MISC_FLAG, initial_data: Option<Vec<u8>>) -> Result<Texture> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: tex_desc.width,
        Height: tex_desc.height,
        MipLevels: 1,
        ArraySize: 1,
        Format: tex_desc.format.into(),
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: usage,
        BindFlags: bind_flags,
        CPUAccessFlags: Default::default(),
        MiscFlags: misc_flag,
    };

    let pitch = tex_desc.width * match tex_desc.format {
        ColorFormat::ARGB8UNorm | ColorFormat::ABGR8UNorm |
        ColorFormat::AYUV | ColorFormat::ARGB10UNorm | ColorFormat::Y410 => {
            Ok(4)
        }
        ColorFormat::YUV444 | ColorFormat::YUV420 | ColorFormat::NV12 => {
            Ok(1)
        }
        ColorFormat::ARGB16Float => {
            Ok(8)
        }
        ColorFormat::YUV444_10bit | ColorFormat::YUV420_10bit => {
            Ok(2)
        }
        _ => {
            Err(DxFilterErr::BadParam("unexpected texture format".to_owned()))
        }
    }?;

    let result = if initial_data.is_some() {
        let init_img = D3D11_SUBRESOURCE_DATA {
            pSysMem: initial_data.unwrap().as_ptr() as _,
            SysMemPitch: pitch,
            SysMemSlicePitch: 0,
        };
        unsafe { device.CreateTexture2D(&desc, &init_img) }
    } else {
        unsafe { device.CreateTexture2D(&desc, null()) }
    };

    if let Err(e) = result {
        Err(DxFilterErr::Unknown(format!("failed to create texture. {:?}", e)))
    } else {
        Ok(Texture::new(result.unwrap()))
    }
}