struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv_coords: vec2<f32>,
    @location(2) linear_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coords: vec2<f32>,
    @location(1) linear_color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(1) @binding(0)
var texture_sampler: sampler;

@group(2) @binding(0)
var texture: texture_2d<f32>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let clip_position = projection * vec4<f32>(input.position, 0.0, 1.0);
    return VertexOutput(clip_position, input.uv_coords, input.linear_color);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(texture, texture_sampler, input.uv_coords);
    return sample * inpur.linear_color;
}
