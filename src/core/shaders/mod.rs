// use handlebars;

pub static GLSL_HDAD: &'static str = "#version 450\n";

pub fn base_vert_str() -> &'static str {
    return include_str!("./base.vert");
}

pub fn base_frag_str() -> &'static str {
    return include_str!("./base.frag");
}

// lazy_static! {
//     let base_vert_str = include_str!("./base.vert");
//     let base_frag_str = include_str!("./base.frag");
// }
