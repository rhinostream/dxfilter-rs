#[cfg(test)]
mod test {
    use core::default::Default;
    use win_desktop_duplication::devices::AdapterFactory;
    use win_desktop_duplication::tex_reader::TextureReader;
    use win_desktop_duplication::texture::Texture;
    use windows::core::Interface;
    use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT, D3D11CreateDevice, ID3D11Device4, ID3D11DeviceContext4};
    use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_1};
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_AYUV, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_NV12, DXGI_SAMPLE_DESC};
    use crate::common_filters::{ConvertARGBToAYUV, ConvertARGBToNV12};
    use crate::DxFilter;

    const SOURCE_IMG: [u8; 1920 * 1080 * 4] = [10; 1920 * 1080 * 4];
    const TARGET_PIX: [u8; 4] = [127, 127, 24, 10];

    #[test]
    fn test_argb_to_ayuv() {
        let adapter = AdapterFactory::new().get_adapter_by_idx(0).unwrap();
        let feature_levels = [D3D_FEATURE_LEVEL_11_1];
        let mut device = None;
        let mut ctx = None;
        let mut level = Default::default();
        unsafe {
            D3D11CreateDevice(
                adapter.as_raw_ref(),
                D3D_DRIVER_TYPE_UNKNOWN,
                None, Default::default(),
                Some(&feature_levels), D3D11_SDK_VERSION, Some(&mut device), Some(&mut level), Some(&mut ctx)).unwrap()
        };

        let ctx: ID3D11DeviceContext4 = ctx.unwrap().cast().unwrap();
        let device: ID3D11Device4 = device.unwrap().cast().unwrap();

        let mut desc = D3D11_TEXTURE2D_DESC {
            Width: 1920,
            Height: 1080,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE,
            CPUAccessFlags: Default::default(),
            MiscFlags: Default::default(),
        };

        let init_pic = D3D11_SUBRESOURCE_DATA {
            pSysMem: SOURCE_IMG.as_ptr() as _,
            SysMemPitch: 1920 * 4,
            SysMemSlicePitch: 0,
        };

        let mut input_tex = None;
        unsafe { device.CreateTexture2D(&desc, Some(&init_pic), Some(&mut input_tex)).unwrap() }
        let input_tex = Texture::new(input_tex.unwrap());

        desc.Format = DXGI_FORMAT_AYUV;
        desc.Width = 1280;
        desc.Height = 720;
        desc.BindFlags = D3D11_BIND_RENDER_TARGET;

        let mut output_tex = None;
        unsafe { device.CreateTexture2D(&desc, None, Some(&mut output_tex)).unwrap() };
        let output_tex = Texture::new(output_tex.unwrap());

        let mut reader = TextureReader::new(device.clone(), ctx.clone());

        let mut filter = ConvertARGBToAYUV::new(&input_tex, &output_tex, &device).unwrap();

        let mut out = Vec::new();

        filter.apply_filter(&ctx).unwrap();

        reader.get_data(&mut out, &output_tex).unwrap();
        assert_eq!(out[0..4], TARGET_PIX);

        filter.set_input_tex(&input_tex).unwrap();
        filter.set_output_tex(&output_tex).unwrap();
        filter.apply_filter(&ctx).unwrap();
        reader.get_data(&mut out, &output_tex).unwrap();
        assert_eq!(out[0..4], TARGET_PIX);
    }

    #[test]
    fn test_argb_to_nv12() {
        let adapter = AdapterFactory::new().get_adapter_by_idx(0).unwrap();
        let feature_levels = [D3D_FEATURE_LEVEL_11_1];
        let mut device = None;
        let mut ctx = None;
        let mut level = Default::default();
        unsafe {
            D3D11CreateDevice(
                adapter.as_raw_ref(),
                D3D_DRIVER_TYPE_UNKNOWN,
                None, Default::default(),
                Some(&feature_levels), D3D11_SDK_VERSION, Some(&mut device), Some(&mut level),
                Some(&mut ctx)).unwrap()
        };

        let ctx: ID3D11DeviceContext4 = ctx.unwrap().cast().unwrap();
        let device: ID3D11Device4 = device.unwrap().cast().unwrap();

        let mut desc = D3D11_TEXTURE2D_DESC {
            Width: 1920,
            Height: 1080,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE,
            CPUAccessFlags: Default::default(),
            MiscFlags: Default::default(),
        };

        let init_pic = D3D11_SUBRESOURCE_DATA {
            pSysMem: SOURCE_IMG.as_ptr() as _,
            SysMemPitch: 1920 * 4,
            SysMemSlicePitch: 0,
        };

        let mut input_tex = None;
        unsafe { device.CreateTexture2D(&desc, Some(&init_pic), Some(&mut input_tex)).unwrap() }
        let input_tex = Texture::new(input_tex.unwrap());

        desc.Format = DXGI_FORMAT_NV12;
        desc.Width = 1280;
        desc.Height = 720;
        desc.BindFlags = D3D11_BIND_RENDER_TARGET;

        let mut output_tex = None;
        unsafe { device.CreateTexture2D(&desc, None, Some(&mut output_tex)).unwrap() }
        let output_tex = Texture::new(output_tex.unwrap());

        let mut reader = TextureReader::new(device.clone(), ctx.clone());

        let mut filter = ConvertARGBToNV12::new(&input_tex, &output_tex, &device).unwrap();

        filter.apply_filter(&ctx).unwrap();

        let mut out = Vec::new();
        reader.get_data(&mut out, &output_tex).unwrap();
        assert_eq!(out.len(), 1280 * 720 * 3 / 2);
        assert_eq!(out[0], TARGET_PIX[2]);
        assert_eq!(out[1280 * 720..(1280 * 720) + 2], TARGET_PIX[0..2]);

        filter.set_input_tex(&input_tex).unwrap();
        filter.set_output_tex(&output_tex).unwrap();
        filter.apply_filter(&ctx).unwrap();
        reader.get_data(&mut out, &output_tex).unwrap();
        assert_eq!(out.len(), 1280 * 720 * 3 / 2);
        assert_eq!(out[0], TARGET_PIX[2]);
        assert_eq!(out[1280 * 720..(1280 * 720) + 2], TARGET_PIX[0..2]);
    }
}


use core::default::Default;
use win_desktop_duplication::texture::{ColorFormat, Texture};
use windows::Win32::Graphics::Direct3D11::{D3D11_COMPARISON_NEVER, D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FLOAT32_MAX, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION_TEXTURE2D, D3D11_SAMPLER_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_TEX2D_RTV, D3D11_TEX2D_SRV, D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_VIEWPORT, ID3D11Device4, ID3D11DeviceContext4, ID3D11RenderTargetView, ID3D11SamplerState, ID3D11ShaderResourceView};
use windows::Win32::Graphics::Direct3D::{D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP, D3D_SRV_DIMENSION_TEXTURE2D};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8_UNORM, DXGI_FORMAT_R8G8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM};
use crate::error::DxFilterErr;
use crate::shader::{PixelShader, VertexShader};
use crate::{DxFilter, Result};

generate_shader!(simple_vs vs {
    src_file: "src\\common_filters\\shaders\\simple_vs.hlsl",
    entry_point: "main",
    target: "vs_5_0"
});

generate_shader!(simple_ps ps {
    src_file: "src\\common_filters\\shaders\\simple_ps.hlsl",
    entry_point: "main",
    target: "ps_5_0"
});

// generate_shader!(argb_to_yuv_pl ps {
//     src_file: "src\\common_filters\\shaders\\argb_to_yuv_pl_ps.hlsl",
//     entry_point: "main",
//     target: "ps_5_0"
// });
generate_shader!(argb_to_ayuv ps {
    src_file: "src\\common_filters\\shaders\\argb_to_ayuv_ps.hlsl",
    entry_point: "main",
    target: "ps_5_0"
});

generate_shader!(argb_to_y ps {
    src_file: "src\\common_filters\\shaders\\argb_to_y_ps.hlsl",
    entry_point: "main",
    target: "ps_5_0"
});

generate_shader!(argb_to_uv ps {
    src_file: "src\\common_filters\\shaders\\argb_to_uv_ps.hlsl",
    entry_point: "main",
    target: "ps_5_0"
});

/// Filter for converting [ARGBUNorm][ColorFormat::ARGB8UNorm] or [ABGRUNorm][ColorFormat::ABGR8UNorm]
/// into [AYUV][ColorFormat::AYUV] format. filter also scales automatically based on input and output textures.
pub struct ConvertARGBToAYUV {
    device: ID3D11Device4,
    vs: VertexShader,
    ps: PixelShader,

    _in_tex: Texture,
    _out_tex: Texture,

    srv: ID3D11ShaderResourceView,
    rtv: ID3D11RenderTargetView,
    sampler: ID3D11SamplerState,
}

impl ConvertARGBToAYUV {
    /// create new instance of ConvertARGBToAYUV filter. After creation, filter takes RGB input from
    /// `input_tex` and writes to AYUV `out_tex`.
    pub fn new(input_tex: &Texture, out_tex: &Texture, device: &ID3D11Device4) -> Result<Self> {
        Self::validate_input(input_tex)?;
        Self::validate_output(out_tex)?;

        let ps = argb_to_ayuv(device.clone())?;
        let vs = simple_vs(device.clone())?;

        let srv = create_srv(device, input_tex, input_tex.desc().format.into())?;
        let sampler = create_tex_sampler(device)?;
        let rtv = create_rtv(device, out_tex, DXGI_FORMAT_R8G8B8A8_UNORM)?;

        return Ok(Self {
            device: device.clone(),
            vs,
            ps,
            _in_tex: input_tex.clone(),
            _out_tex: out_tex.clone(),
            srv,
            rtv,
            sampler,
        });
    }


    fn validate_input(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::ARGB8UNorm | ColorFormat::ABGR8UNorm => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected ARGB or ABGR format found {:?}", desc.format).to_owned()))
            }
        }
    }
    fn validate_output(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::AYUV => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected AYUV format found {:?}", desc.format).to_owned()))
            }
        }
    }
}

impl DxFilter for ConvertARGBToAYUV {
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()> {
        let out_desc = self._out_tex.desc();
        let vp = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: out_desc.width as _,
            Height: out_desc.height as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };
        unsafe {
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            ctx.VSSetShader(self.vs.as_raw_ref(), Some(&[]));
            ctx.PSSetShader(self.ps.as_raw_ref(), Some(&[]));
            ctx.PSSetSamplers(0, Some(&[self.sampler.clone()]));
            ctx.PSSetShaderResources(0, Some(&[self.srv.clone()]));
            ctx.RSSetViewports(Some(&[vp]));
            ctx.OMSetRenderTargets(Some(&[self.rtv.clone()]), None);
            ctx.Draw(4, 0);
        }
        return Ok(());
    }

    fn set_input_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToAYUV::validate_input(tex)?;
        self._in_tex = tex.clone();
        self.srv = create_srv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }

    fn set_output_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToAYUV::validate_output(tex)?;
        self._out_tex = tex.clone();
        self.rtv = create_rtv(&self.device, tex, DXGI_FORMAT_R8G8B8A8_UNORM)?;
        return Ok(());
    }
}


/// Filter for converting [ARGBUNorm][ColorFormat::ARGB8UNorm] or [ABGRUNorm][ColorFormat::ABGR8UNorm]
/// into [NV12][ColorFormat::NV12] format. filter also scales automatically based on input and output textures.
pub struct ConvertARGBToNV12 {
    device: ID3D11Device4,
    vs: VertexShader,
    y_ps: PixelShader,
    uv_ps: PixelShader,

    _in_tex: Texture,
    _out_tex: Texture,

    srv: ID3D11ShaderResourceView,
    rtv_y: ID3D11RenderTargetView,
    rtv_uv: ID3D11RenderTargetView,

    sampler: ID3D11SamplerState,
}

impl ConvertARGBToNV12 {
    /// create new instance of ConvertARGBToANV12 filter. After creation, filter takes RGB input from
    /// `input_tex` and writes to NV12 `out_tex`.
    pub fn new(input_tex: &Texture, out_tex: &Texture, device: &ID3D11Device4) -> Result<Self> {
        Self::validate_input(input_tex)?;
        Self::validate_output(out_tex)?;
        let y_ps = argb_to_y(device.clone())?;
        let uv_ps = argb_to_uv(device.clone())?;
        let vs = simple_vs(device.clone())?;

        let srv = create_srv(device, input_tex, input_tex.desc().format.into())?;
        let sampler = create_tex_sampler(device)?;
        let rtv_y = create_rtv(device, out_tex, DXGI_FORMAT_R8_UNORM)?;
        let rtv_uv = create_rtv(device, out_tex, DXGI_FORMAT_R8G8_UNORM)?;

        return Ok(Self {
            device: device.clone(),
            vs,
            y_ps,
            uv_ps,
            _in_tex: input_tex.clone(),
            _out_tex: out_tex.clone(),
            srv,
            rtv_y,
            rtv_uv,
            sampler,
        });
    }

    fn validate_input(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::ARGB8UNorm | ColorFormat::ABGR8UNorm => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected ARGB or ABGR format found {:?}", desc.format).to_owned()))
            }
        }
    }
    fn validate_output(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::NV12 => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected NV12 format found {:?}", desc.format).to_owned()))
            }
        }
    }
}

impl DxFilter for ConvertARGBToNV12 {
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()> {
        let out_desc = self._out_tex.desc();
        let vp_y = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: out_desc.width as _,
            Height: out_desc.height as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };
        let vp_uv = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: (out_desc.width / 2) as _,
            Height: (out_desc.height / 2) as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };

        unsafe {
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            ctx.VSSetShader(self.vs.as_raw_ref(), Some(&[]));
            ctx.PSSetShader(self.y_ps.as_raw_ref(), Some(&[]));
            ctx.PSSetSamplers(0, Some(&[self.sampler.clone()]));
            ctx.PSSetShaderResources(0, Some(&[self.srv.clone()]));
            ctx.RSSetViewports(Some(&[vp_y]));
            ctx.OMSetRenderTargets(Some(&[self.rtv_y.clone()]), None);
            ctx.Draw(4, 0);
            ctx.PSSetShader(self.uv_ps.as_raw_ref(), Some(&[]));
            ctx.RSSetViewports(Some(&[vp_uv]));
            ctx.OMSetRenderTargets(Some(&[self.rtv_uv.clone()]), None);
            ctx.Draw(4, 0);
        }
        return Ok(());
    }

    fn set_input_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToNV12::validate_input(tex)?;
        self._in_tex = tex.clone();
        self.srv = create_srv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }

    fn set_output_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToNV12::validate_output(tex)?;
        self._out_tex = tex.clone();
        self.rtv_y = create_rtv(&self.device, tex, DXGI_FORMAT_R8_UNORM)?;
        self.rtv_uv = create_rtv(&self.device, tex, DXGI_FORMAT_R8G8_UNORM)?;
        return Ok(());
    }
}


/// Filter for simple scaling of [ARGBUNorm][ColorFormat::ARGB8UNorm] or [ABGRUNorm][ColorFormat::ABGR8UNorm] or [AYUV][ColorFormat::AYUV]
/// formats.
pub struct ScaleARGBOrAYUV {
    device: ID3D11Device4,
    vs: VertexShader,
    ps: PixelShader,

    _in_tex: Texture,
    _out_tex: Texture,

    srv: ID3D11ShaderResourceView,
    rtv: ID3D11RenderTargetView,
    sampler: ID3D11SamplerState,
}

impl ScaleARGBOrAYUV {
    /// create new instance of ScaleARGBOrAYUV filter. After creation, filter takes ARGB or ABGR or AYUV input from
    /// `input_tex` and writes to same format `out_tex` after scaling.
    pub fn new(input_tex: &Texture, out_tex: &Texture, device: &ID3D11Device4) -> Result<Self> {
        Self::validate_input(input_tex)?;
        Self::validate_output(out_tex)?;

        let ps = simple_ps(device.clone())?;
        let vs = simple_vs(device.clone())?;

        let srv = create_srv(device, input_tex, DXGI_FORMAT_R8G8B8A8_UNORM)?;
        let sampler = create_tex_sampler(device)?;
        let rtv = create_rtv(device, out_tex, DXGI_FORMAT_R8G8B8A8_UNORM)?;

        return Ok(Self {
            device: device.clone(),
            vs,
            ps,
            _in_tex: input_tex.clone(),
            _out_tex: out_tex.clone(),
            srv,
            rtv,
            sampler,
        });
    }


    fn validate_input(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::ARGB8UNorm | ColorFormat::ABGR8UNorm | ColorFormat::AYUV => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected ARGB or ABGR format found {:?}", desc.format).to_owned()))
            }
        }
    }
    fn validate_output(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::AYUV | ColorFormat::ABGR8UNorm | ColorFormat::ARGB8UNorm => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected AYUV format found {:?}", desc.format).to_owned()))
            }
        }
    }
}

impl DxFilter for ScaleARGBOrAYUV {
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()> {
        let out_desc = self._out_tex.desc();
        let vp = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: out_desc.width as _,
            Height: out_desc.height as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };
        unsafe {
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            ctx.VSSetShader(self.vs.as_raw_ref(), Some(&[]));
            ctx.PSSetShader(self.ps.as_raw_ref(), Some(&[]));
            ctx.PSSetSamplers(0, Some(&[self.sampler.clone()]));
            ctx.PSSetShaderResources(0, Some(&[self.srv.clone()]));
            ctx.RSSetViewports(Some(&[vp]));
            ctx.OMSetRenderTargets(Some(&[self.rtv.clone()]), None);
            ctx.Draw(4, 0);
        }
        return Ok(());
    }

    fn set_input_tex(&mut self, tex: &Texture) -> Result<()> {
        ScaleARGBOrAYUV::validate_input(tex)?;
        self._in_tex = tex.clone();
        self.srv = create_srv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }

    fn set_output_tex(&mut self, tex: &Texture) -> Result<()> {
        ScaleARGBOrAYUV::validate_output(tex)?;
        self._out_tex = tex.clone();
        self.rtv = create_rtv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }
}

/// Filter for converting [ARGBUNorm][ColorFormat::ARGB8UNorm] or [ABGRUNorm][ColorFormat::ABGR8UNorm]
/// into [YUV444][ColorFormat::YUV444] format. filter also scales automatically based on input and output textures.
pub struct ConvertARGBToYUV444 {
    device: ID3D11Device4,
    vs: VertexShader,
    ps: PixelShader,

    _in_tex: Texture,
    _out_tex: Texture,

    srv: ID3D11ShaderResourceView,
    rtv: ID3D11RenderTargetView,
    sampler: ID3D11SamplerState,
}

impl ConvertARGBToYUV444 {
    /// create new instance of ConvertARGBToYUV444 filter. After creation, filter takes ARGB or ABGR input from
    /// `input_tex` and writes to YUV444 format `out_tex` after scaling.
    pub fn new(input_tex: &Texture, out_tex: &Texture, device: &ID3D11Device4) -> Result<Self> {
        Self::validate_input(input_tex)?;
        Self::validate_output(out_tex)?;

        let ps = simple_ps(device.clone())?;
        let vs = simple_vs(device.clone())?;

        let srv = create_srv(device, input_tex, DXGI_FORMAT_R8G8B8A8_UNORM)?;
        let sampler = create_tex_sampler(device)?;
        let rtv = create_rtv(device, out_tex, DXGI_FORMAT_R8_UNORM)?;

        return Ok(Self {
            device: device.clone(),
            vs,
            ps,
            _in_tex: input_tex.clone(),
            _out_tex: out_tex.clone(),
            srv,
            rtv,
            sampler,
        });
    }


    fn validate_input(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::ARGB8UNorm | ColorFormat::ABGR8UNorm => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected ARGB or ABGR format found {:?}", desc.format).to_owned()))
            }
        }
    }
    fn validate_output(tex: &Texture) -> Result<()> {
        let desc = tex.desc();
        match desc.format {
            ColorFormat::YUV444 => {
                Ok(())
            }
            _ => {
                Err(DxFilterErr::BadParam(format!("expected AYUV format found {:?}", desc.format).to_owned()))
            }
        }
    }
}

impl DxFilter for ConvertARGBToYUV444 {
    fn apply_filter(&self, ctx: &ID3D11DeviceContext4) -> Result<()> {
        let out_desc = self._out_tex.desc();
        let vp = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: out_desc.width as _,
            Height: (out_desc.height * 3) as _,
            MinDepth: 0.0,
            MaxDepth: 0.0,
        };
        unsafe {
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            ctx.VSSetShader(self.vs.as_raw_ref(), Some(&[]));
            ctx.PSSetShader(self.ps.as_raw_ref(), Some(&[]));
            ctx.PSSetSamplers(0, Some(&[self.sampler.clone()]));
            ctx.PSSetShaderResources(0, Some(&[self.srv.clone()]));
            ctx.RSSetViewports(Some(&[vp]));
            ctx.OMSetRenderTargets(Some(&[self.rtv.clone()]), None);
            ctx.Draw(4, 0);
        }
        return Ok(());
    }

    fn set_input_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToYUV444::validate_input(tex)?;
        self._in_tex = tex.clone();
        self.srv = create_srv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }

    fn set_output_tex(&mut self, tex: &Texture) -> Result<()> {
        ConvertARGBToYUV444::validate_output(tex)?;
        self._out_tex = tex.clone();
        self.rtv = create_rtv(&self.device, tex, tex.desc().format.into())?;
        return Ok(());
    }
}

fn create_srv(dev: &ID3D11Device4, tex: &Texture, format: DXGI_FORMAT) -> Result<ID3D11ShaderResourceView> {
    let mut srv_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D_SRV_DIMENSION_TEXTURE2D,
        Anonymous: Default::default(),
    };
    srv_desc.Anonymous.Texture2D = D3D11_TEX2D_SRV {
        MostDetailedMip: 0,
        MipLevels: 1,
    };
    let mut srv = None;
    if let Err(e) = unsafe { dev.CreateShaderResourceView(tex.as_raw_ref(), Some(&srv_desc), Some(&mut srv)) } {
        Err(DxFilterErr::Unknown(format!("failed to create shader resource view. {:?}", e)))
    } else {
        Ok(srv.unwrap())
    }
}

fn create_rtv(dev: &ID3D11Device4, tex: &Texture, format: DXGI_FORMAT) -> Result<ID3D11RenderTargetView> {
    let mut rtv_desc = D3D11_RENDER_TARGET_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D,
        Anonymous: Default::default(),
    };
    rtv_desc.Anonymous.Texture2D = D3D11_TEX2D_RTV {
        MipSlice: 0,
    };
    let mut rtv = None;

    if let Err(e) = unsafe { dev.CreateRenderTargetView(tex.as_raw_ref(), Some(&rtv_desc), Some(&mut rtv)) } {
        Err(DxFilterErr::Unknown(format!("failed to create render target view. {:?}", e)))
    } else {
        Ok(rtv.unwrap())
    }
}

fn create_tex_sampler(dev: &ID3D11Device4) -> Result<ID3D11SamplerState> {
    let sampler_desc = D3D11_SAMPLER_DESC {
        Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
        AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
        MipLODBias: 0.0,
        MaxAnisotropy: 0,
        ComparisonFunc: D3D11_COMPARISON_NEVER,
        BorderColor: [255f32; 4],
        MinLOD: 0.0,
        MaxLOD: D3D11_FLOAT32_MAX,
    };
    let mut sampler_state = None;
    let sampler = unsafe { dev.CreateSamplerState(&sampler_desc, Some(&mut sampler_state)) };

    if let Err(e) = sampler {
        Err(DxFilterErr::Unknown(format!("failed to create shader resource view. {:?}", e)))
    } else {
        Ok(sampler_state.unwrap())
    }
}