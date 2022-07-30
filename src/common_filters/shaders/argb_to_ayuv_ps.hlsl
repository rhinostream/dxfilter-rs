//--------------------------------------------------------------------------------------
// argb_to_ayuv_ps.hlsl
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
float4 main(PS_INPUT input) :SV_Target
{
	float4 InputColor = txInput.Sample(GenericSampler, input.Tex);

	// Range 0-255
	float ColorY = (0.257f * InputColor.r + 0.504f * InputColor.g + 0.098f * InputColor.b) + (16 / 256.0f);

	// Range 0-255
	float ColorU = (-0.148f * InputColor.r - 0.291f * InputColor.g + 0.439f * InputColor.b) + (128.0f / 256.0f);
	float ColorV = (0.439f * InputColor.r - 0.368f * InputColor.g - 0.071f * InputColor.b) + (128.0f / 256.0f);

	ColorY = clamp(ColorY, 0.0f, 255.0f);
	ColorU = clamp(ColorU, 0.0f, 255.0f);
	ColorV = clamp(ColorV, 0.0f, 255.0f);

	return float4(ColorV,ColorU,ColorY,InputColor.a);
}
