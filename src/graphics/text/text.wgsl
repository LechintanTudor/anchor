struct Instance {
    @location(0) bounds_edges: vec4<f32>,
    @location(1) tex_coords_edges: vec4<f32>,
    @location(2) linear_color: vec4<f32>,
    @location(3) scale_rotation_col_0: vec2<f32>,
    @location(4) scale_rotation_col_1: vec2<f32>,
    @location(5) translation: vec2<f32>,
}

struct Vertex {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) linear_color: vec4<f32>,
}

var<private> EDGE_INDEXES: array<array<u32, 2>, 6> = array<array<u32, 2>, 6>(
    array<u32, 2>(1u, 0u), // left, top
    array<u32, 2>(1u, 2u), // left, bottom
    array<u32, 2>(3u, 0u), // right, top
    array<u32, 2>(3u, 0u), // right, top
    array<u32, 2>(1u, 2u), // left, bottom
    array<u32, 2>(3u, 2u), // right, bottom
);

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(0) @binding(1)
var sprite_sheet: texture_2d<f32>;

@group(0) @binding(2)
var sprite_sheet_sampler: sampler;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, instance: Instance) -> Vertex {
    let edge_indexes = EDGE_INDEXES[i];
    let scale_rotation = mat2x2<f32>(instance.scale_rotation_col_0, instance.scale_rotation_col_1);

    let untransformed_position = vec2<f32>(
        instance.bounds_edges[edge_indexes[0]],
        instance.bounds_edges[edge_indexes[1]],
    );
    let position = scale_rotation * untransformed_position + instance.translation;
    let clip_position = projection * vec4<f32>(position, 0.0, 1.0);

    let tex_coords = vec2<f32>(
        instance.tex_coords_edges[edge_indexes[0]],
        instance.tex_coords_edges[edge_indexes[1]],
    );

    return Vertex(clip_position, tex_coords, instance.linear_color);
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
    let opacity = textureSample(sprite_sheet, sprite_sheet_sampler, vertex.tex_coords).r;
    return vec4<f32>(1.0, 1.0, 1.0, opacity) * vertex.linear_color;
}
