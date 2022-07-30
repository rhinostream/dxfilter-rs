#[macro_export]
macro_rules! generate_shader {
    ($name:ident ps $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::PixelShader> {
            $crate::shader::PixelShader::new(&$crate::shader_compile! $content, device)
        }
    };
    ($name:ident vs $content: tt) => {
        fn $name (device: $crate::ID3D11Device4) -> $crate::Result<$crate::shader::VertexShader> {
            $crate::shader::VertexShader::new(&$crate::shader_compile! $content, device)
        }
    };
}

