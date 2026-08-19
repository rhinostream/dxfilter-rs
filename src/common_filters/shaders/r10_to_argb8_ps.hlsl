//--------------------------------------------------------------------------------------
// r10_to_argb8_ps.hlsl
//
// Converts a R10G10B10A2_UNORM source into R8G8B8A8_UNORM (8-bit ARGB).
// R10G10B10A2 as produced by DXGI desktop duplication is already sRGB-encoded
// (display-referred), so this is a passthrough: the hardware SRV unpacks the
// 10-10-10-2 pixels into float4 in [0,1] and writing to an R8G8B8A8 RTV truncates
// to 8 bits. No OETF is applied here (applying one would double-gamma the image).
// Alpha is forced opaque: the 2-bit source alpha is too quantized to be useful
// downstream and HDR desktop frames are reliably opaque.
//--------------------------------------------------------------------------------------
Texture2D txInput : register(t0);

SamplerState GenericSampler : register(s0);

struct PS_INPUT
{
    float4 Pos : SV_POSITION;
    float2 Tex : TEXCOORD;
};

//--------------------------------------------------------------------------------------
// Pixel Shader
//--------------------------------------------------------------------------------------
float4 main(PS_INPUT input) : SV_Target
{
    float4 InputColor = txInput.Sample(GenericSampler, input.Tex);

    return float4(InputColor.r, InputColor.g, InputColor.b, 1.0f);
}