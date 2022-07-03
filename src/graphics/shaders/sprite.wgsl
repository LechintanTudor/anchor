struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> ortho_matrix: mat4x4<f32>;

@group(0) @binding(1)
var texture: texture_2d<f32>;

@group(0) @binding(2)
var texture_sampler: sampler;


@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let clip_position = ortho_matrix * vec4<f32>(input.position, 0.0, 1.0);
    return VertexOutput(clip_position, input.color, input.tex_coords);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, input.tex_coords) * input.color;
}
