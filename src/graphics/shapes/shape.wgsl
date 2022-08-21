struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) scale_rotation_col_0: vec2<f32>,
    @location(2) scale_rotation_col_1: vec2<f32>,
    @location(3) translation: vec2<f32>,
    @location(4) pixel_anchor: vec2<f32>,
    @location(5) linear_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) linear_color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let scale_rotation = mat2x2<f32>(input.scale_rotation_col_0, input.scale_rotation_col_1);
    let position = scale_rotation * (input.position - input.pixel_anchor) + input.translation;
    let clip_position = projection * vec4<f32>(position, 0.0, 1.0);

    return VertexOutput(clip_position, input.linear_color);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.linear_color);
}
