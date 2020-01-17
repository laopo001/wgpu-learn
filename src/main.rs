#![allow(unused_imports)]

use wgpu_learn::{
    app,
    config::{Config, Event},
    console_log, time,
};

fn main() {
    let mut app = app::App::new("123", Config::PowerHighPerformance);
    dbg!(app.get_info_adapter());
    app.on(
        Event::Start,
        Box::new(|| {
            dbg!("start");
        }),
    );
    app.start();
}
