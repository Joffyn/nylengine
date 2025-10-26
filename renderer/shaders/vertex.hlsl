struct VSInput
{
    float3 pos : POSITION;
    float3 color : COLOR;
};

struct VSOut {
    float4 position : SV_Position;
    float4 color : COLOR0;
};

VSOut main(VSInput input) {
    VSOut output;
    output.position = float4(input.pos, 1.0);
    output.color = float4(input.color, 1.0);
    return output;
}
