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

use hexmap;


/*
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
*/

fn drawmap(canvas: &mut render::WindowCanvas, block_texture: &render::Texture, map: &hexmap::Hexmap) {
    canvas.set_draw_color(Color::RGB(0,32,128));
    canvas.clear();
    let mut y = 128;
    let mut sx: i32 = 616;
    /*
    for row in map.tiles.iter() {
        let mut offset = false;
        let mut x = 128;
        for col in row.iter() {
           let texturerow = match col {
               &hexmap::TerrainKind::Stone => 0
           };
           if offset {
               canvas.copy(&block_texture, Rect::new(0,texturerow,256,192), Rect::new(x,y+16,64,48)).expect("Render failed");
           } else {
               canvas.copy(&block_texture, Rect::new(0,texturerow,256,192), Rect::new(x,y,64,48)).expect("Render failed");
           }
           x=x+48;
           offset=!offset;
        }
        y=y+48;
    }
    */
    for rank in 0..13 {
        let map = map.get_ranked_map();
        let sx=640;
        let sy=400;
        
        for &(offset,hex) in map.iter() {
            match hex { // fixme -- don't draw here. just set texturerow
                &hexmap::TerrainKind::Stone => canvas.copy(&block_texture, Rect::new(0,0,256,192), Rect::new(sx+offset.0*32,sy+offset.1*32,64,48)).expect("Render failed"),
                &hexmap::TerrainKind::Ocean => println!("I dunno how to do nothing"),
            }
        }
    }
}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump, islandmap: &mut hexmap::Hexmap) {

    let texture_creator = canvas.texture_creator();
    let block_texture = texture_creator.load_texture("/home/mattdm/misc/island/images/hexblocks.png").unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1280, 800).unwrap();

    // fill the background
    canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
        drawmap(texture_canvas, &block_texture, &islandmap);
    }).unwrap();


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
