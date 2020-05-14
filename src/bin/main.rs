#![allow(unused)]
#[macro_use]
extern crate lazy_static;
#[cfg(not(target_arch = "wasm32"))]
use async_std::task;
use std::sync::Mutex;
use wgpu_learn::util::console_log;
use wgpu_learn::{
    app,
    config::{Config, Event},
    time,
};
// lazy_static! {
//     /// This is an example for using doc comment attributes
//     static ref ARR: Mutex<Vec<i32>> = Mutex::new(vec![]);
// }

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;

    // app.on(Event::Start, |app| {
    //     ARR.lock().unwrap().push(1);
    // });
    // app.on(Event::End, |app| {
    //     dbg!(ARR.lock().unwrap());
    // });

    app.on(Event::Start, |app| {
        app.array.push(1);
    });
    app.on(Event::End, |app| {
        dbg!(app.array.len());
    });
    app.start();
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    task::block_on(run());
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
}
