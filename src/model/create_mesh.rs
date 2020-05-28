use crate::app::App;
use crate::config::Attrib;
use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex::Vertex;
use crate::core::vertex_buffer::VertexBuffer;
use crate::core::vertex_format::{VertexFormat, VertexType};
use crate::model::mesh::Mesh;
use crate::Vector3;
use cgmath::prelude::*;
use cgmath::Zero;
use zerocopy::{AsBytes, FromBytes};

#[allow(non_upper_case_globals)]
static primitiveUv1Padding: f32 = 4.0 / 64.0;
#[allow(non_upper_case_globals)]
static primitiveUv1PaddingScale: f32 = 1.0 - primitiveUv1Padding * 2.0;

pub fn create_box(
    app: &App,
    half_extents: Option<Vector3>,
    width_segments: Option<u32>,
    length_segments: Option<u32>,
    height_segments: Option<u32>,
) -> Mesh {
    let he = half_extents.unwrap_or(Vector3::new(0.5, 0.5, 0.5));
    let ws = width_segments.unwrap_or(1);
    let ls = length_segments.unwrap_or(1);
    let hs = height_segments.unwrap_or(1);
    let corners = [
        Vector3::new(-he.x, -he.y, he.z),
        Vector3::new(he.x, -he.y, he.z),
        Vector3::new(he.x, he.y, he.z),
        Vector3::new(-he.x, he.y, he.z),
        Vector3::new(he.x, -he.y, -he.z),
        Vector3::new(-he.x, -he.y, -he.z),
        Vector3::new(-he.x, he.y, -he.z),
        Vector3::new(he.x, he.y, -he.z),
    ];

    let faceAxes: [[i32; 3]; 6] = [
        [0, 1, 3], // FRONT
        [4, 5, 7], // BACK
        [3, 2, 6], // TOP
        [1, 0, 4], // BOTTOM
        [1, 4, 2], // RIGHT
        [5, 0, 6], // LEFT
    ];

    let faceNormals: [[i32; 3]; 6] = [
        [0, 0, 1],  // FRONT
        [0, 0, -1], // BACK
        [0, 1, 0],  // TOP
        [0, -1, 0], // BOTTOM
        [1, 0, 0],  // RIGHT
        [-1, 0, 0], // LEFT
    ];
    let mut positions = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut uvs1 = vec![];
    let mut indices = vec![];
    let mut vcounter = 0;
    let mut generateFace = |side: usize, uSegments: u32, vSegments: u32| {
        let mut u;
        let mut v;

        for i in 0..=uSegments {
            for j in 0..=vSegments {
                let mut temp1 = Vector3::zero();
                let mut temp2 = Vector3::zero();
                let mut temp3 = Vector3::zero();
                let mut r = Vector3::zero();

                temp1 = corners[faceAxes[side][0] as usize]
                    .lerp(corners[faceAxes[side][1] as usize], (i / uSegments) as f32);

                temp2 = corners[faceAxes[side][0] as usize]
                    .lerp(corners[faceAxes[side][2] as usize], (j / vSegments) as f32);
                temp3 = (temp2 - corners[faceAxes[side][0] as usize]);
                r = (temp1 + temp3);
                u = (i / uSegments) as f32;
                v = (j / vSegments) as f32;

                positions.extend_from_slice(&[r.x, r.y, r.z]);
                normals.extend_from_slice(&[
                    faceNormals[side][0] as f32,
                    faceNormals[side][1] as f32,
                    faceNormals[side][2] as f32,
                ]);
                uvs.extend_from_slice(&[u, v]);
                // pack as 3x2
                // 1/3 will be empty, but it's either that or stretched pixels
                // TODO: generate non-rectangular lightMaps, so we could use space without stretching
                u /= 3.0;
                v /= 3.0;
                u = u * primitiveUv1PaddingScale + primitiveUv1Padding;
                v = v * primitiveUv1PaddingScale + primitiveUv1Padding;
                u += (side as f32 % 3.0) / 3.0;
                v += (side as f32 / 3.0).floor() / 3.0;
                uvs1.extend_from_slice(&[u, v]);

                if ((i < uSegments) && (j < vSegments)) {
                    indices.extend_from_slice(&[vcounter + vSegments + 1, vcounter + 1, vcounter]);
                    indices.extend_from_slice(&[
                        vcounter + vSegments + 1,
                        vcounter + vSegments + 2,
                        vcounter + 1,
                    ]);
                }

                vcounter += 1;
            }
        }
    };
    generateFace(0, ws, hs);
    generateFace(1, ws, hs);
    generateFace(2, ws, ls);
    generateFace(3, ws, ls);
    generateFace(4, ls, hs);
    generateFace(5, ls, hs);
    // crate::console_log!(&positions);
    return create_mesh(
        app,
        CreateMeshParam {
            positions,
            normals: Some(normals),
            uvs: Some(uvs),
            uvs1: Some(uvs1),
            colors: None,
            indices: Some(indices),
        },
    );
}

struct CreateMeshParam {
    positions: Vec<f32>,
    normals: Option<Vec<f32>>,
    colors: Option<Vec<f32>>,
    uvs: Option<Vec<f32>>,
    uvs1: Option<Vec<f32>>,
    indices: Option<Vec<u32>>,
}

fn create_mesh(app: &App, param: CreateMeshParam) -> Mesh {
    let mut mesh = Mesh::new(&app);
    let mut vertex_type_vec = vec![];
    vertex_type_vec.push(VertexType {
        attrib: Attrib::POSITION,
        size: 3,
    });
    if param.colors.is_some() {
        vertex_type_vec.push(VertexType {
            attrib: Attrib::COLOR,
            size: 3,
        });
    }
    if param.normals.is_some() {
        vertex_type_vec.push(VertexType {
            attrib: Attrib::NORMAL,
            size: 3,
        });
    }
    if param.uvs.is_some() {
        vertex_type_vec.push(VertexType {
            attrib: Attrib::TEXCOORD0,
            size: 2,
        });
    }
    let format = VertexFormat::new(vertex_type_vec);

    let vertex_num = (param.positions.len() as f32 / 3.0) as usize;
    let mut vertex_data = vec![];

    for i in 0..vertex_num {
        vertex_data.push(Vertex {
            position: [
                param.positions[i * 3],
                param.positions[i * 3 + 1],
                param.positions[i * 3 + 2],
            ],
            color: param
                .colors
                .as_ref()
                .map(|vec| [vec[i * 3], vec[i * 3 + 1], vec[i * 3 + 2]]),
            tex_coord: param.uvs.as_ref().map(|vec| [vec[i * 2], vec[i * 2 + 1]]),
            normal: param
                .normals
                .as_ref()
                .map(|vec| [vec[i * 3], vec[i * 3 + 1], vec[i * 3 + 2]]),
        });
    }
    let vertex_data = vertex_data
        .iter()
        .map(|x| {
            return x.data();
        })
        .collect::<Vec<Box<[f32]>>>()
        .concat();

    let vertex_buffer = VertexBuffer::new(vertex_data.as_bytes().to_vec(), format);
    mesh.set_vertex_buffer(vertex_buffer);
    // crate::console_log!(param.indices.as_ref().unwrap());
    if let Some(indices) = param.indices {
        let index_buffer = IndexBuffer::new(indices.as_bytes().to_vec(), indices.len());
        mesh.set_index_buffer(index_buffer);
    }

    return mesh;
}
