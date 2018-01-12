extern crate sdl2;

// The event loop handles... events -- and also basic drawing.
    
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;


use sdl2::video;
use sdl2::render;

use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;

use landscape;
use direction::Direction;
use sprite::SpriteAtlas;

fn drawmap(canvas: &mut render::WindowCanvas, sprite_atlas: &SpriteAtlas, map: &landscape::Island, orientation: Direction, zoom: i32) {
    canvas.set_draw_color(Color::RGB(0,112,160));
    canvas.clear();

    // these should be actual center minus half a hex
    let center_x=960-32;
    let center_y=540-24;
    
    let drawstart = time::Instant::now();

    let map = map.get_ranked(orientation);
    println!("  Got Ranked {:?}: {}",orientation,(time::Instant::now()-drawstart).subsec_nanos()/1000000);


    for &(offset,hexstack) in map.iter() {
    
        // long term improvement: read this from a 
        // text file describing the texture, rather
        // than hard-coding.
        
        if hexstack.is_some() {
            let mut elevation=0;
            for tile in hexstack.unwrap().iter() {
                //canvas.copy(&sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*160,256,160), Rect::new(center_x+offset.0*32,center_y+offset.1*24-elevation*8,64,40)).expect("Render failed");
                //fixme: don't hardcode elevation (or scale!)
                // zoom is easy!
                sprite_atlas.draw(canvas, tile, zoom as u32, center_x+offset.0*128/zoom,center_y+offset.1*96/zoom-elevation*32/zoom,orientation);

                elevation += 1;
            }
            // test kludge for the tree sprite
            if elevation==1 {
                // FIXME: "draw-offset should be in sprite (but private to that sprite)
                sprite_atlas.draw(canvas, "tree-palm", zoom as u32, center_x+offset.0*128/zoom+16/zoom,center_y+offset.1*96/zoom-elevation*32/zoom-160/zoom,orientation);
            }
        }
    }
    println!("  Map drawn:  {}",(time::Instant::now()-drawstart).subsec_nanos()/1000000);

    
    // Draw compass rose.    
    // FIXME: I _think_ this should be part of an "interface" layer, not the background.
    // (But I might be wrong)
    // FIXME: same deal about hardcoding the location here
    //canvas.copy(&sprite_sheet, Rect::new(texturecol*256,1536,256,96), Rect::new(1664,968,256,96)).expect("Render failed");
    sprite_atlas.draw(canvas, "compass", 1, 1664, 968,orientation);    

    println!("  Compass:    {}",(time::Instant::now()-drawstart).subsec_nanos()/1000000);

}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump) {


    let texture_creator = canvas.texture_creator();

    // load the sprite atlas
    //let sprite_atlas = SpriteAtlas::new(&texture_creator);
    let sprite_atlas = SpriteAtlas::new(&texture_creator);
        
    // this is what the background gets rendered onto
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 1920, 1080).unwrap();

    // create the map. in the future, we probably want some game-setup
    // function first before we go right into the game loop
    let mut islandmap = landscape::Island::new();
    
    // start 100 ms ago, so that we go right into drawing at the
    // bottom of the loop    
    let mut event_ticker = time::Instant::now() - time::Duration::from_millis(100);
    let mut frame_ticker = event_ticker;
    
    // FIXME: add more sophisticated data structure for interface state
    // like zoom and stuff too
    let mut orientation=Direction::SE; // FIXME: use a diagonal to start?
    let mut zoom=4;
    
    
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
                Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
                    if zoom > 1 {
                        zoom = zoom/2;
                        background_refresh_needed = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if zoom < 32 {
                        zoom = zoom*2;
                        background_refresh_needed = true;
                    }
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
                    //canvas.set_viewport(Rect::new(0,0,960,540));
                },
                Event::Window {win_event,..} => {
                    match win_event {
                        WindowEvent::SizeChanged(wx,_wy) => {
                            // Keep 16×9 aspect ratio
                            // FIXME: this doesn't really work (leaves strip of desktop in fullscreen!)
                            // Nneed to change the copy call instead
                            canvas.set_viewport(Rect::new(0,0,wx as u32,((wx as u32)*9)/16));
                        },
                        _ => { /* println!("{:?}",win_event); */ }
                    }
                    //println!("{:?}",win_event);
                
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
                    drawmap(texture_canvas, &sprite_atlas, &islandmap, orientation, zoom);
                }).unwrap();
                background_refresh_needed = false;
                println!("Background Refesh Total: {}",(time::Instant::now()-now).subsec_nanos()/1000000);
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
