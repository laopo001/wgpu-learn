#![allow(unused_imports)]

use wgpu_learn::{app, config, console_log, time};

fn main() {
    let app = app::App::new("123", config::Config::PowerHighPerformance);
    dbg!(app.get_info_adapter());
    app.start();
}
