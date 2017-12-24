extern crate sdl2;

// The event loop handles... events -- and also basic drawing.


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;



pub fn gameloop(canvas: &mut sdl2::render::WindowCanvas, event_pump: &mut sdl2::EventPump) {


    let texture_creator = canvas.texture_creator();
    let block_texture = texture_creator.load_texture("/home/mattdm/misc/island/images/hexblocks.png").unwrap();

    canvas.set_draw_color(Color::RGB(0,32,128));
    canvas.clear();

    canvas.copy(&block_texture, Rect::new(0,0,256,256), Rect::new(608,368,64,64)).expect("Render failed");
    canvas.copy(&block_texture, Rect::new(0,0,256,256), Rect::new(656,384,64,64)).expect("Render failed");
    canvas.copy(&block_texture, Rect::new(0,0,256,256), Rect::new(608,400,64,64)).expect("Render failed");

    canvas.present();
    
    let mut i = 0;
    
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    i=i+1;
                    if i>5 {
                        i=0
                    };
                    canvas.copy(&block_texture, Rect::new(i*256,0,256,256), Rect::new(608,368,64,64)).expect("Render failed");
                    canvas.copy(&block_texture, Rect::new(i*256,0,256,256), Rect::new(656,384,64,64)).expect("Render failed");
                    canvas.copy(&block_texture, Rect::new(i*256,0,256,256), Rect::new(608,400,64,64)).expect("Render failed");
                    canvas.present();
                },
                _ => {}
            }
        }
        
        // The rest of the game loop goes here...
        
        
        


        thread::sleep(time::Duration::from_millis(10));
    }
}
