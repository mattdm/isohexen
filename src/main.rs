extern crate sdl2;
extern crate rand;

//use std::thread;

mod interface;
mod hexgeometry;
mod landscape;
//mod sprite;

pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::INIT_PNG).unwrap();

    let window = video_subsystem.window("Little Island", 960, 540)
        //.fullscreen_desktop()
        //.resizable()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    interface::gameloop(&mut canvas,&mut event_pump)
}
