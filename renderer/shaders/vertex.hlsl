StructuredBuffer<float2> positions : register(t0);

struct VSOut {
    float4 position : SV_Position;
    float4 color : COLOR0;
};

VSOut main(uint vertex_id : SV_VertexID) {
    VSOut output;
    float2 pos = positions[vertex_id];
    output.position = float4(pos, 0.0, 1.0);
    output.color = float4(1, 0, 0, 1);
    return output;
}
