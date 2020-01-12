[[vk::binding(0, 0)]]
Texture2D shaderTexture;
[[vk::binding(1, 0)]]
SamplerState SampleType;

struct PixelInputType
{
    float4 position : SV_POSITION;
    float2 tex : TEXCOORD0;
};

float4 TexturePixelShader(PixelInputType input) : SV_TARGET
{
    float4 textureColor, textureColor2;

    textureColor = shaderTexture.Sample(SampleType, input.tex);

    return textureColor;
}

