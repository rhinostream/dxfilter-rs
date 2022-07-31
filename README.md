# DxFilter

[![docs.rs](https://img.shields.io/docsrs/dxfilter)](https://docs.rs/dxfilter)
[![Crates.io](https://img.shields.io/crates/v/dxfilter)](https://crates.io/crates/dxfilter)
[![Crates.io](https://img.shields.io/crates/l/dxfilter)](https://crates.io/crates/dxfilter)

Scale and ColorConversion done with DirectX filters. You can also create your
own filters with the provided api.

Crate contains various tools to make these features possible.

- `generate_shader!` and `compile_shader!` macros to write shaders that compile at compile time
- various built filters for converting and scaling from RGB to YUV or NV12
- `DxFilter` interface for writing custom filters
- utils like `create_device_context` , `create_input_tex`, `create_output_tex` for easier setup.
- utils like `AdapterFactory`, `Adapter`, `TextureReader` imported
  from [`win_desktop_duplication`](https://crates.io/crates/win_desktop_duplication).

__For example usage, look at examples/rgb_to_nv12.rs__

## Usage

```rust
// for more detailed example see examples/rgb_to_nv12.rs
fn main() {
    // {...}

    //                                    Texture    Texture    directx device
    let filter = ConvertARGBToNV12::new(&input_tex, &output_tex, &device).unwrap();
    //                directx device
    filter.apply_filter(&context).unwrap();

    // { ... }
}
```

## AvailableFilters

* [x] ARGB to AYUV
* [x] ARGB to NV12
* [x] ARGB or AYUV scale only
* [ ] ARGB to YUV planar
* [ ] ARGB to YUV420 planar
* [ ] ARGB16 to Y410
* [ ] ARGB16 to YUV444 10bit planar
* [ ] ARGB16 to YUV420 10bit planar