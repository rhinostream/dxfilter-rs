// examples/rgb_to_nv12.rs
use dxfilter::utils::{create_input_texture, create_output_texture, create_device_context};
use dxfilter::{ConvertARGBToNV12, DxFilter};
use win_desktop_duplication::devices::AdapterFactory;
use win_desktop_duplication::tex_reader::TextureReader;
use win_desktop_duplication::texture::{ColorFormat, TextureDesc};

fn main() {
    // create device and context

    let adapter = AdapterFactory::new().get_adapter_by_idx(0).unwrap();
    let (device, context) = create_device_context(&adapter).unwrap();

    // create input and output textures
    let sample_input_data = vec![10; 1920 * 1080 * 4];
    let input_tex = create_input_texture(
        &device,
        TextureDesc {
            height: 1080,
            width: 1920,
            format: ColorFormat::ARGB8UNorm,
        }, Some(sample_input_data)).unwrap();
    let output_tex = create_output_texture(
        &device,
        TextureDesc {
            height: 720,
            width: 1280,
            format: ColorFormat::NV12,
        }, None).unwrap();

    // initialize texture reader
    let mut reader = TextureReader::new(device.clone(), context.clone());

    // now to the main event
    let filter = ConvertARGBToNV12::new(&input_tex, &output_tex, &device).unwrap();

    filter.apply_filter(&context).unwrap();

    let mut out_data = Vec::new();
    reader.get_data(&mut out_data, &output_tex).unwrap();

    println!("output data size is: {}", out_data.len());
}