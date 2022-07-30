//--------------------------------------------------------------------------------------
// simple_ps.hlsl
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

	return float4(InputColor.r,InputColor.g,InputColor.b,InputColor.a);
}
