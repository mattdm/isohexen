extern crate sdl2;

// The event loop handles... events -- and also basic drawing.


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::render;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;

fn drawabunchofhexes(canvas: &mut render::WindowCanvas, block_texture: &render::Texture) {
    canvas.set_draw_color(Color::RGB(0,32,128));
    canvas.clear();
    for y in 0..21 {
        for x in 0..12 {
            canvas.copy(&block_texture, Rect::new(0,0,256,192), Rect::new(64+x*96,48+y*32,64,48)).expect("Render failed");
        }
        for x in 0..12 {
            canvas.copy(&block_texture, Rect::new(0,0,256,192), Rect::new(112+x*96,64+y*32,64,48)).expect("Render failed");
        }
    }
}


pub fn gameloop(mut canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump) {

    let texture_creator = canvas.texture_creator();
    let block_texture = texture_creator.load_texture("/home/mattdm/misc/island/images/hexblocks.png").unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1280, 800).unwrap();

    // fill the background
    canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
        drawabunchofhexes(texture_canvas, &block_texture);
    });


    let mut event_ticker = time::Instant::now();
    let mut frame_ticker = event_ticker;
    let mut tnow = event_ticker;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                _ => {}
            }
        }
        
        // The rest of the game loop goes here...

        // Approximately 20fps        
        let next_tick = frame_ticker + time::Duration::from_millis(50);
        let now = time::Instant::now();
        if now >= next_tick {
            canvas.copy(&background_texture, None, None).expect("Render failed");
            canvas.present();
            println!("draw");
            frame_ticker = next_tick;
        }


        // but sleep around 10ms for event loop, because that's responsive enough
        let next_tick = event_ticker + time::Duration::from_millis(10);
        let now = time::Instant::now();
        println!("{:7}", now.duration_since(tnow).subsec_nanos()/1000);
        tnow = now;
        if now < next_tick {
            thread::sleep(next_tick-now);
            println!("sleep")
        }
        event_ticker = next_tick;
    }
}
