macro_rules! vertex_attr_array {
    ($ty:ty { $($location:expr => $field:ident: $field_ty:ident,)* }) => {{
        let vertex: $ty = Default::default();

        [$(
            ::wgpu::VertexAttribute {
                format: ::wgpu::VertexFormat::$field_ty,
                offset: ::bytemuck::offset_of!(vertex, $ty, $field) as _,
                shader_location: $location
            },
        )*]
    }};
}

pub(crate) use vertex_attr_array;
