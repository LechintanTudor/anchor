struct Instance {
    @location(0) size: vec2<f32>,
    @location(1) translation: vec2<f32>,
    @location(2) tex_coords_edges: vec4<f32>,
    @location(3) linear_color: vec4<f32>,
}

struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) linear_color: vec4<f32>,
}

var<private> TEX_COORDS_EDGE_INDEXES: array<array<u32, 2>, 6> = array<array<u32, 2>, 6>(
    array<u32, 2>(1u, 0u), // left, top
    array<u32, 2>(1u, 2u), // left, bottom
    array<u32, 2>(3u, 0u), // right, top
    array<u32, 2>(3u, 0u), // right, top
    array<u32, 2>(1u, 2u), // left, bottom
    array<u32, 2>(3u, 2u), // right, bottom
);

var<private> CORNERS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-0.5, -0.5), // left, top
    vec2<f32>(-0.5, 0.5),  // left, bottom
    vec2<f32>(0.5, -0.5),  // right, top
    vec2<f32>(0.5, -0.5),  // right, top
    vec2<f32>(-0.5, 0.5),  // left, bottom
    vec2<f32>(0.5, 0.5),   // right, bottom
);

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(0) @binding(1)
var sprite_sheet: texture_2d<f32>;

@group(0) @binding(2)
var sprite_sheet_sampler: sampler;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, instance: Instance) -> Vertex {
    let absolute_position = instance.translation + instance.size * CORNERS[i];
    let position = projection * vec4<f32>(absolute_position, 0.0, 1.0);

    let tex_coords_edge_indexes = TEX_COORDS_EDGE_INDEXES[i];
    let tex_coords = vec2<f32>(
        instance.tex_coords_edges[tex_coords_edge_indexes[0]],
        instance.tex_coords_edges[tex_coords_edge_indexes[1]],
    );

    return Vertex(position, tex_coords, instance.linear_color);
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
    let texture_sample = textureSample(
        sprite_sheet,
        sprite_sheet_sampler,
        vertex.tex_coords
    );

    return texture_sample * vertex.linear_color;
}
