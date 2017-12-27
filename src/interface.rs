extern crate sdl2;

// The event loop handles... events -- and also basic drawing.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::render;

use sdl2::image::LoadTexture;
//use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;
use std::path;

use hexmap;


fn drawmap(canvas: &mut render::WindowCanvas, block_texture: &render::Texture, map: &hexmap::Hexmap) {
    canvas.set_draw_color(Color::RGB(0,112,160));
    canvas.clear();

    // FIXME: don't hardcode values
    
    // these should be actual center minus half a hex
    let center_x=640-32;
    let center_y=400-24;

    let map = map.get_ranked();

    for &(offset,hex) in map.iter() {
    
        let tilerow = match hex { // fixme -- don't draw here. just set texturerow
            &hexmap::TerrainKind::Stone => Some(0),
            &hexmap::TerrainKind::Sand  => Some(1),
            &hexmap::TerrainKind::Dirt  => Some(2),
            &hexmap::TerrainKind::Ocean => None, 
        };
        if tilerow.is_some() {
            canvas.copy(&block_texture, Rect::new(0,tilerow.unwrap()*48,64,48), Rect::new(center_x+offset.0*32,center_y+offset.1*24,64,48)).expect("Render failed");
        }
    }

   //canvas.copy(&block_texture, Rect::new(0,0,64,48), Rect::new(640,570,64,48)).expect("Render failed");
}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump, islandmap: &mut hexmap::Hexmap) {


    let texture_creator = canvas.texture_creator();
    let block_texture = texture_creator.load_texture(path::Path::new("images/hexblocks.png")).unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1280, 800).unwrap();
    
    // fill the background
    canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
        drawmap(texture_canvas, &block_texture, &islandmap);
    }).unwrap();

    canvas.set_draw_color(Color::RGB(0,32,128));
    canvas.clear();
    canvas.copy(&block_texture,  Rect::new(0,0,64,48),  Rect::new(0,0,64,48)).expect("Render failed");
    canvas.present();
    
    let mut event_ticker = time::Instant::now();
    let mut frame_ticker = event_ticker;

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
            // FIXME draw animations here
            canvas.present();
            frame_ticker = next_tick;
        }


        // but sleep around 10ms for event loop, because that's responsive enough
        let next_tick = event_ticker + time::Duration::from_millis(10);
        let now = time::Instant::now();
        if now < next_tick {
            thread::sleep(next_tick-now);
        }
        event_ticker = next_tick;
    }
}
