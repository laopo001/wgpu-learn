#![feature(fn_traits)]
#![allow(non_snake_case)]
#![allow(unused)]
#[macro_use]
extern crate lazy_static;
// pub enum ShaderStage {
//     Vertex,
//     Fragment,
//     Compute,
// }

pub fn Texture2TextureCopyView<'a>(t: &'a wgpu::Texture) -> wgpu::TextureCopyView<'a> {
    wgpu::TextureCopyView {
        texture: t,
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
    }
}
#[macro_export]
macro_rules! console_log {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("{:?}",$x);
            )*
        }
    };
}

#[macro_export]
macro_rules! extends {
    ($a:ident,$b:ty, $( {$k:ident: $v:ty} ),* ) => {
        pub struct $a {
            pub __parent: $b,
            $(
                pub $k: $v,
            )*
        }
        use core::ops::Deref;
        impl Deref for $a {
            type Target = $b;
            fn deref<'a>(&'a self) -> &'a $b {
                &self.__parent
            }
        }
    };
}

pub mod time {
    pub fn now() -> u128 {
        return std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }
}

pub mod app;
pub mod config;
pub mod core;
pub mod model;
pub mod shaders;
pub mod util;
pub type Matrix4 = cgmath::Matrix4<f32>;
pub type Vector4 = cgmath::Vector4<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector2 = cgmath::Vector2<f32>;
pub type Color3 = Vector3;
pub type Color4 = Vector4;
pub type Color = wgpu::Color;
pub type VertexFormat = wgpu::VertexFormat;
// pub type ShaderStage = glsl_to_spirv::ShaderType;
