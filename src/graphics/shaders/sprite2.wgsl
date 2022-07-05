struct Instance {
    @location(0) sprite_sheet_size: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) anchor: vec2<f32>,
    @location(3) scale_rotation_col_0: vec2<f32>,
    @location(4) scale_rotation_col_1: vec2<f32>,
    @location(5) translation: vec2<f32>,
    @location(6) absolute_tex_coords_edges: vec4<f32>, // top, left, bottom, right
    @location(7) linear_color: vec4<f32>,
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
var<uniform> ortho_matrix: mat4x4<f32>;

@group(1) @binding(0)
var sprite_sheet: texture_2d<f32>;

@group(1) @binding(1)
var sprite_sheet_sampler: sampler;

@vertex
fn vs_main(@builtin(vertex_index) i: u32, instance: Instance) -> Vertex {
    var position: vec4<f32>;

    {
        let scale_rotation_matrix = mat2x2<f32>(
            instance.scale_rotation_col_0,
            instance.scale_rotation_col_1,
        );
        let relative_position = instance.size * (CORNERS[i] - instance.anchor);
        let absolute_position = (scale_rotation_matrix * relative_position + instance.translation);
        position = ortho_matrix * vec4<f32>(absolute_position, 0.0, 1.0);
    }

    var tex_coords: vec2<f32>;

    {
        let tex_coords_edge_indexes = TEX_COORDS_EDGE_INDEXES[i];
        let absolute_tex_coords_edges = vec2<f32>(
            instance.absolute_tex_coords_edges[tex_coords_edge_indexes[0]],
            instance.absolute_tex_coords_edges[tex_coords_edge_indexes[1]],
        );
        tex_coords = absolute_tex_coords_edges / instance.sprite_sheet_size;
    }

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
