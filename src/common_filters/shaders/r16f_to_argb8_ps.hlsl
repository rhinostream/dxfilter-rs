//--------------------------------------------------------------------------------------
// r16f_to_argb8_ps.hlsl
//
// Converts a R16G16B16A16_FLOAT (linear scRGB, as produced by DXGI desktop
// duplication on an HDR display) source into R8G8B8A8_UNORM (8-bit sRGB ARGB).
// scRGB values are linear-light and may exceed 1.0 (HDR highlights). The simple
// tonemap requested for this step is: saturate to [0,1] (hard-clip highlights)
// then apply the sRGB OETF (IEC 61966-2-1 transfer function) to get gamma-encoded
// 8-bit output. Alpha is forced opaque.
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

    // Clamp linear-light scRGB to [0,1] (simple HDR->SDR tonemap: hard-clip).
    float3 c = saturate(InputColor.rgb);

    // sRGB OETF (linear -> sRGB encoded). Component-wise ternary for float3.
    float3 lo = 12.92f * c;
    float3 hi = 1.055f * pow(c, 1.0f / 2.4f) - 0.055f;
    float3 encoded = (c <= 0.0031308f) ? lo : hi;

    return float4(encoded, 1.0f);
}