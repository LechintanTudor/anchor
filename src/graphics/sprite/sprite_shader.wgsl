struct VertexInput {
    @location(0) size: vec2<f32>,
    @location(1) scale_rotation_x_axis: vec2<f32>,
    @location(2) scale_rotation_y_axis: vec2<f32>,
    @location(3) translation: vec2<f32>,
    @location(4) anchor_offset: vec2<f32>,
    @location(5) uv_edges: vec4<f32>,
    @location(6) linear_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coords: vec2<f32>,
    @location(1) linear_color: vec4<f32>,
}

var<private> EDGE_INDEXES: array<array<u32, 2>, 4> = array<array<u32, 2>, 4>(
    array<u32, 2>(1u, 0u), // left, top
    array<u32, 2>(1u, 2u), // left, bottom
    array<u32, 2>(3u, 0u), // right, top
    array<u32, 2>(3u, 2u), // right, bottom
);

var<private> CORNERS: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    vec2<f32>(0.0, 0.0), // left, top
    vec2<f32>(0.0, 1.0), // left, bottom
    vec2<f32>(1.0, 0.0), // right, top
    vec2<f32>(1.0, 1.0), // right, bottom
);

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(1) @binding(0)
var texture_sampler: sampler;

@group(2) @binding(0)
var texture: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, input: VertexInput) -> VertexOutput {
    let scale_rotation = mat2x2<f32>(
        input.scale_rotation_x_axis,
        input.scale_rotation_y_axis,
    );
    
    let position = scale_rotation
        * (input.size * CORNERS[i] - input.anchor_offset)
        + input.translation;
        
    let clip_position = projection * vec4<f32>(position, 0.0, 1.0);

    let uv_indexes = EDGE_INDEXES[i];

    let uv_coords = vec2<f32>(
        input.uv_edges[uv_indexes[0]],
        input.uv_edges[uv_indexes[1]],
    );

    return VertexOutput(clip_position, uv_coords, input.linear_color);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(
        texture,
        texture_sampler,
        input.uv_coords,
    );
    
    return sample * input.linear_color;
}
