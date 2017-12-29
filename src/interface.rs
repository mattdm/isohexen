extern crate sdl2;

// The event loop handles... events -- and also basic drawing.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use sdl2::render;

use sdl2::image::LoadTexture;
//use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;
use std::path;

use hexmap;


fn drawmap(canvas: &mut render::WindowCanvas, sprite_sheet: &render::Texture, map: &hexmap::Hexmap, orientation: hexmap::Direction) {
    canvas.set_draw_color(Color::RGB(0,112,160));
    canvas.clear();

    // FIXME: don't hardcode values
    
    // these should be actual center minus half a hex
    let center_x=640-32;
    let center_y=400-24;

    let map = map.get_ranked(orientation);

    let texturecol = match orientation {
        hexmap::Direction::E  => 0,
        hexmap::Direction::SE => 1,
        hexmap::Direction::SW => 2,
        hexmap::Direction::W  => 3,
        hexmap::Direction::NW => 4,
        hexmap::Direction::NE => 5,
    };




    for &(offset,hex) in map.iter() {
    
        // long term improvement: read this from a 
        // text file describing the texture, rather
        // than hard-coding.
        let texturerow = match hex {
            &hexmap::TerrainKind::Stone => Some(0),
            &hexmap::TerrainKind::Sand  => Some(1),
            &hexmap::TerrainKind::Dirt  => Some(2),
            &hexmap::TerrainKind::Grass  => Some(3),
            &hexmap::TerrainKind::Ocean => None, 
        };
        if texturerow.is_some() {
            // fixme: also don't hardcode texture width/height
            canvas.copy(&sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*192,256,192), Rect::new(center_x+offset.0*32,center_y+offset.1*24,64,48)).expect("Render failed");
        }
    }
    
    // FIXME: I _think_ this should be part of an "interface" layer, not the background.
    // (But I might be wrong)
    // FIXME: same deal about hardcoding the location here
    canvas.copy(&sprite_sheet, Rect::new(texturecol*256,768,256,192), Rect::new(1096,648,256,192)).expect("Render failed");
    

}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump, islandmap: &mut hexmap::Hexmap) {


    let texture_creator = canvas.texture_creator();
    let sprite_sheet = texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1280, 800).unwrap();
    
    // fill the background

    canvas.set_draw_color(Color::RGB(0,32,128));
    canvas.clear();
    canvas.present();
    
    let mut event_ticker = time::Instant::now();
    let mut frame_ticker = event_ticker;
    
    // FIXME: add more sophisticated data structure for interface state
    // like zoom and stuff too
    let mut orientation=hexmap::Direction::E; // FIXME: use a diagonal to start?
    let mut background_refresh_needed = true;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                /* Planning to use AWEDXZ for panning in approriate
                   direction, so let's use Q and R for rotation. */
                Event::KeyUp { keycode: Some(Keycode::Q), .. } |
                Event::KeyUp { keycode: Some(Keycode::PageUp), .. } => {
                    orientation = orientation.counterclockwise();
                    background_refresh_needed = true;
                },
                Event::KeyUp { keycode: Some(Keycode::R), .. } |
                Event::KeyUp { keycode: Some(Keycode::PageDown), .. } => {
                    orientation = orientation.clockwise();
                    background_refresh_needed = true;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x: mx, y: my, .. } => {
                    if mx > 1096 && my > 648 {
                        orientation = orientation.counterclockwise();
                        background_refresh_needed = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x: mx, y: my,.. } => {
                    if mx > 1096 && my > 648 {
                        orientation = orientation.clockwise();
                        background_refresh_needed = true;
                    }
                },
                _ => {}
            }
        }
        
        // The rest of the game loop goes here...

        // Approximately 20fps        
        let next_tick = frame_ticker + time::Duration::from_millis(50);
        let now = time::Instant::now();
        if now >= next_tick {
            if background_refresh_needed {
                canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
                    drawmap(texture_canvas, &sprite_sheet, &islandmap, orientation);
                }).unwrap();
                background_refresh_needed = false;
            }

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
