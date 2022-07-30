/// Compile DirectX shader at compile time and generates a function. The function takes an
/// `ID3D11Device4` instance to produce respective shader.
///
/// ## Syntax:
///
/// ```
///  //                             ps for pixel shader       syntax for compile_shader! check
///  // name of func to create      vs for vertex shader      compile_shader docs for this syntax
///  //        |_________________        |      ____________________|
///  //                           \      |     /
///             generate_shader!(fn_name ps {...})
/// ```
///
/// ## Example Usage:
///
/// ```rust
///
///  use dx11_screencap::generate_shader;
///
///
///  // this directly translates to
///  // fn sample_pixel_shader(device: ID3D11Device4)->PixelShader { ... }
///  generate_shader!(sample_pixel_shader ps {///
///     src: "
///         int main() {
///             return 1;
///         }
///     ",
///     entry_point: "main"
///     target: "ps_5_0"
///  });
///
///  fn main(){
///     // somehow acquire directx device
///     let sample_shader = sample_pixel_shader(device);
///  }
///
/// ```
///
#[macro_export]
macro_rules! generate_shader {
    ($name:ident ps $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::PixelShader> {
            $crate::shader::PixelShader::new(&$crate::compile_shader! $content, device)
        }
    };
    ($name:ident vs $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::VertexShader> {
            $crate::shader::VertexShader::new(&$crate::compile_shader! $content, device)
        }
    };
}

