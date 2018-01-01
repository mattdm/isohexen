extern crate sdl2;

// The event loop handles... events -- and also basic drawing.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use sdl2::render;
use sdl2::video;

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
    let center_x=960-32;
    let center_y=540-24;

    let map = map.get_ranked(orientation);

    let texturecol = match orientation {
        hexmap::Direction::E  => 0,
        hexmap::Direction::SE => 1,
        hexmap::Direction::SW => 2,
        hexmap::Direction::W  => 3,
        hexmap::Direction::NW => 4,
        hexmap::Direction::NE => 5,
    };

    for &(offset,hexstack) in map.iter() {
    
        // long term improvement: read this from a 
        // text file describing the texture, rather
        // than hard-coding.
        
        if hexstack.is_some() {
            let mut elevation=0;
            for tile in hexstack.unwrap().iter() {
                let texturerow = match tile {
                    &hexmap::TerrainKind::Stone => Some(0),
                    &hexmap::TerrainKind::Sand  => Some(1),
                    &hexmap::TerrainKind::Dirt  => Some(2),
                    &hexmap::TerrainKind::Grass  => Some(3),
                    //&hexmap::TerrainKind::Ocean => None, 
                };
                if texturerow.is_some() {
                    // FIXME: don't hardcode texture width/height
                    // FIXME: make elevation multiplier a parameter. 8 means columns are smooth
                    // and higher values give different looks. 10 and 12 are good.	
                    canvas.copy(&sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*160,256,160), Rect::new(center_x+offset.0*32,center_y+offset.1*24-elevation*8,64,40)).expect("Render failed");
                    // palm tree test kludge
                    if elevation==0 && texturerow==Some(1) && offset.0 & 2 == 0  && offset.1 & 2 == 0{
                        canvas.copy(&sprite_sheet, Rect::new(0,640,256,256), Rect::new(center_x+offset.0*32-24,center_y+offset.1*24-elevation*8-108,128,128)).expect("Render failed");
                    }
                }
                elevation += 1;
            }
        }
    }
    
    // Draw compass rose.    
    // FIXME: I _think_ this should be part of an "interface" layer, not the background.
    // (But I might be wrong)
    // FIXME: same deal about hardcoding the location here
    canvas.copy(&sprite_sheet, Rect::new(texturecol*256,1536,256,96), Rect::new(1664,968,256,96)).expect("Render failed");
    

}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump, islandmap: &mut hexmap::Hexmap) {


    let texture_creator = canvas.texture_creator();
    let sprite_sheet = texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1920, 1080).unwrap();
    
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
    
    islandmap.generate();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                /* Planning to use AWEDXZ for panning in approriate
                   direction, so let's use Q and R for rotation. */
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageUp), .. } => {
                    orientation = orientation.counterclockwise();
                    background_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => {
                    orientation = orientation.clockwise();
                    background_refresh_needed = true;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x: mx, y: my, .. } => {
                    if mx > 1112 && my > 688 {
                        orientation = orientation.counterclockwise();
                        background_refresh_needed = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x: mx, y: my,.. } => {
                    if mx > 1112 && my > 688 {
                        orientation = orientation.clockwise();
                        background_refresh_needed = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    islandmap.generate();
                    background_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    match canvas.window_mut().fullscreen_state() {
                        video::FullscreenType::Off => canvas.window_mut().set_fullscreen(video::FullscreenType::Desktop).unwrap(),
                        video::FullscreenType::Desktop => canvas.window_mut().set_fullscreen(video::FullscreenType::Off).unwrap(),
                        video::FullscreenType::True => unreachable!(),
                    };
                },

                _ => {}
            }
        }
        
        // The rest of the game loop goes here...

        // Approximately 20fps        
        let next_tick = frame_ticker + time::Duration::from_millis(50);
        let now = time::Instant::now(); // fixme: better to call this only once per loop, but
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
