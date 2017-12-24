extern crate sdl2;

//use std::thread;

mod gameloop;


pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Little Island", 1280, 800)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    gameloop::gameloop(&mut canvas,&mut event_pump)

}
