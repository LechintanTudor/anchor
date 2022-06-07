struct CameraUniform {
    ortho: mat4x4<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

[[stage(vertex)]]
fn vs_main(input: VertexInput) -> VertexOutput {
    let position = camera.ortho * vec4<f32>(input.position, 0.0, 1.0);
    return VertexOutput(position, input.color);
}

[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.color);
}
