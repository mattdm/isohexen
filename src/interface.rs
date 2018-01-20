extern crate sdl2;

// The event loop handles... events -- and also basic drawing.
    
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
//use sdl2::keyboard::Scancode;
use sdl2::mouse;
use sdl2::mouse::MouseButton;

use sdl2::surface;
use sdl2::image::LoadSurface;
use sdl2::video;
use sdl2::render;

use sdl2::rect::Rect;
use sdl2::rect::Point;

use sdl2::pixels::Color;

use sdl2::ttf;

use std::time;
use std::thread;
use std::cmp;

use std::collections::HashSet;


use landscape;
use direction::Direction;
use sprite::SpriteAtlas;

fn letterbox(w: i32, h: i32) -> Rect {
    if w/16 < h/9 { //letterbox
        let nh = (w*9)/16;
        return Rect::new(0,(h-nh)/2,w as u32,nh as u32);
    } if w/16 > h/9 { // pillarbox
        let nw = (h*16)/9;
        return Rect::new((w-nw)/2,0,nw as u32,h as u32);
    } else { // 16:9, so take the whole thign
        return Rect::new(0,0,w as u32,h as u32);
   }
}

fn point_to_hex(p: Point,map_x: i32, map_y: i32, zoom: i32) {
    // basic algorithm: pixels per hex offset so the center is 0,0


    // same calculations as when actually drawing the map
    let visible_w=1920/4*zoom;
    let visible_h=1080/4*zoom;
    let world_x = 16384/2-visible_w/2+((map_x*(16384-visible_w))/2048);  // 2048 is our scroll range
    let world_y = 8640/2 -visible_h/2+((map_y*(8640 -visible_h))/2048);
    

    // here, a zoomed-in hex is 256 pixels wide, and we want half that
    // remember that zoom has a 4x multiplier, so that's why that is here
    let subcolumn_number=(((p.x*zoom)/4)+world_x)/128;
    let subcolumn_offset=(((p.x*zoom)/4)+world_x)%128;
    // zoomed-in hex is 160 pixels high, but we're offset, so 96 is the magic number
    let gridrow_number=(((p.y*zoom)/4)+world_y)/96;
    let row_offset=(((p.y*zoom)/4)+world_y)%96;
    let mut offset_column_number=0;
    let row_number;

    if row_offset < 32 {
        // This means we're in the area where two hex diagonals come together.
        // And the following handy logic corresponds to the hex border slope 
        // direction — true if sloped like \, false if like /
        if gridrow_number%2 == subcolumn_number%2 { 
            // slope goes down
            if subcolumn_offset > 0 && 128*row_offset/subcolumn_offset < 32 {
                row_number=gridrow_number-51;
            } else {
                row_number=gridrow_number-50;
            }
            
        } else {
            // slope goes up
            if 128*row_offset/(128-subcolumn_offset) < 32 {
                row_number=gridrow_number-51;
            } else {
                row_number=gridrow_number-50;
            }
        }
    } else {
        // The easy case of the "rectangle" part of each hex row
        row_number=gridrow_number-50;
    }
    if row_number%2 == 0 {
        offset_column_number = (subcolumn_number+1)/2-32;
    } else {
        offset_column_number = subcolumn_number/2-32;
    }

    //println!("M({},{}) -> C{}({}) R{}",p.x,p.y, offset_column_number, row_number, row_number);
    println!("M({},{}) -> C{} R{}",p.x,p.y, offset_column_number, row_number);
}

fn draw_background(canvas: &mut render::WindowCanvas, sprite_atlas: &SpriteAtlas) {

    // sky
    canvas.set_draw_color(Color::RGB(96,192,208));
    canvas.clear();

    // sea
    let horizon=1288;//1096;
    for y in 0..39 {
        for x in 0..64 {
            sprite_atlas.draw(canvas, "ocean", 1, x*256,y*192+horizon,Direction::E);
        }
        for x in 0..65 {
            sprite_atlas.draw(canvas, "ocean", 1, x*256-128,y*192+horizon+96,Direction::E);
        }
    }
}    

fn draw_map(canvas: &mut render::WindowCanvas, background: &render::Texture, sprite_atlas: &SpriteAtlas, map: &landscape::Island, orientation: Direction) {

    canvas.copy(background, None, None).expect("Render failed");

    // x is the actual center; y is pushed down for the sky
    let center_x=8192-128;
    let center_y=4800;
    
    //let drawstart = time::Instant::now();

    let map = map.get_ranked(orientation);
    //println!("  Got Ranked {:?}: {}",orientation,(time::Instant::now()-drawstart).subsec_nanos()/1000000);


    for &(offset,hexstack,decorstack) in map.iter() {
    
        // long term improvement: read this from a 
        // text file describing the texture, rather
        // than hard-coding.
        
        let mut elevation=0;
        if hexstack.is_some() {
            for tile in hexstack.unwrap().iter() {
                //canvas.copy(&sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*160,256,160), Rect::new(center_x+offset.0*32,center_y+offset.1*24-elevation*8,64,40)).expect("Render failed");
                //fixme: don't hardcode elevation (or scale!)
                sprite_atlas.draw(canvas, tile, 1, center_x+offset.0*128,center_y+offset.1*96-elevation*32,orientation);
                elevation += 1;
            }
        }
        if decorstack.is_some() {
            for decor in decorstack.unwrap().iter() {
                // FIXME: "draw-offset should be in sprite (but private to that sprite)
                sprite_atlas.draw(canvas, decor, 1, center_x+offset.0*128,center_y+offset.1*96-elevation*32,orientation);
                elevation += 1;
            }
        }
        
    }
    //println!("  Map drawn:  {}",(time::Instant::now()-drawstart).subsec_nanos()/1000000);
}

pub fn splash(canvas: &mut render::WindowCanvas) {
    let (lw,lh)=canvas.logical_size();
    canvas.set_logical_size(1920,1080).unwrap();
    
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(50,116,153));
    canvas.clear();
    let ttf_context = ttf::init().unwrap();
    let overpass_regular = ttf_context.load_font("fonts/overpass-regular.otf", 144).unwrap();
    let overpass_light   = ttf_context.load_font("fonts/overpass-light.otf",    72).unwrap();
    let words = overpass_regular.render("LITTLE ISLAND")
                        .blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let splash = texture_creator.create_texture_from_surface(&words).unwrap();
    canvas.copy(&splash,None,Rect::new(1920/2-words.width() as i32/2,380,words.width(),words.height())).unwrap();
    let words = overpass_light.render("A DEMO FOR ISOHEXEN BY MATTHEW MILLER")
                        .blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let splash = texture_creator.create_texture_from_surface(&words).unwrap();
    canvas.copy(&splash,None,Rect::new(1920/2-words.width() as i32/2,580,words.width(),words.height())).unwrap();
    canvas.present();
    
    canvas.set_logical_size(lw,lh).unwrap();
}


pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump, mouse_util: &mouse::MouseUtil) {

    mouse_util.show_cursor(false);

    let mut draw_rect = letterbox(canvas.window().size().0 as i32,canvas.window().size().1 as i32);

    let texture_creator = canvas.texture_creator();
    

    // FIXME: move to mouse-cursor specific functions
    let cursor_surface = match surface::Surface::from_file("images/cursor.png") {
        // OH LOOK ERROR HANDLING
        Ok(cursor_surface) => cursor_surface,
        Err(err)    => panic!("Couldn't find cursor image file: {}", err)
    };
    let cursor = match mouse::Cursor::from_surface(cursor_surface, 0, 0) {
        Ok(cursor) => cursor,
        Err(err) => panic!("Could not set cursor: {}", err)
    };
    cursor.set();
    

    // load the sprite atlas
    let sprite_atlas = SpriteAtlas::new(&texture_creator,"images/spritesheet.toml");

    // this is what the scene gets rendered onto 
    // FIXME: put these constants somewhere as constants.
    // FIXME: all of this goes into the setup function
    //   ... and then we can leave the mouse stuff all in main()
    let mut world_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 16384, 8640).unwrap();
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 16384, 8640).unwrap();

    // create the map. in the future, we probably want some game-setup
    // function first before we go right into the game loop
    let mut islandmap = landscape::Island::new();
    
    
    
    let mut event_ticker = time::Instant::now()  - time::Duration::from_millis(1000);
    let mut frame_ticker = event_ticker;
    
    // FIXME: add more sophisticated data structure for interface state
    // like zoom and stuff too
    let mut orientation=Direction::SE;
    let mut map_x = 0;
    let mut map_y = 0;
    let mut zoom=1; // fixme start at 32
    
    let mut fullscreen_refresh_needed = 1; // need to repeat because of some weird race condition
    let mut world_refresh_needed = true;
    let mut background_refresh_needed = true;
    
    islandmap.generate(64);



    'mainloop: loop {
        let keys: HashSet<Keycode> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        /*
        let mouse_buttons: HashSet<MouseButton> = event_pump.mouse_state().pressed_mouse_buttons().collect();
        */
        let mouse_x = (event_pump.mouse_state().x()*1920)/draw_rect.width()  as i32 - draw_rect.x();
        let mouse_y = (event_pump.mouse_state().y()*1080)/draw_rect.height() as i32 - draw_rect.y();
        //println!("{:?}",keys.contains(&Keycode::O));
        //println!("{},{}",mouse_x,mouse_y);
        point_to_hex(Point::new(mouse_x,mouse_y),map_x,map_y,zoom);


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                /* AWEDXZ for panning in hex directions */
                Event::KeyDown { keycode: Some(Keycode::A), .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    // West
                    map_x -= 10*zoom;
                    map_x = cmp::max(map_x,-1024);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    // East
                    map_x += 10*zoom;
                    map_x = cmp::min(map_x, 1024);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // North West
                    if keys.contains(&Keycode::E) { // straight down
                        map_y -= 8*zoom;
                        map_y = cmp::max(map_y,-1024);
                    } else {
                        map_x -= 8*zoom;
                        map_y -= 5*zoom;
                        map_x = cmp::max(map_x,-1024);
                        map_y = cmp::max(map_y,-1024);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    // North East
                    if keys.contains(&Keycode::W) { // straight down
                        map_y -= 8*zoom;
                        map_y = cmp::max(map_y,-1024);
                    } else {
                        map_x += 8*zoom;
                        map_y -= 5*zoom;
                        map_x = cmp::min(map_x, 1024);
                        map_y = cmp::max(map_y,-1024);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    // South West
                    if keys.contains(&Keycode::X) { // straight up
                        map_y += 8*zoom;
                        map_y = cmp::min(map_y, 1024);
                    } else {
                        map_x -= 8*zoom;
                        map_y += 5*zoom;
                        map_x = cmp::max(map_x,-1024);
                        map_y = cmp::min(map_y, 1024);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    // South East
                    if keys.contains(&Keycode::Z) { // straight up
                        map_y += 8*zoom;
                        map_y = cmp::min(map_y, 1024);
                    } else {
                        map_x += 8*zoom;
                        map_y += 5*zoom;
                     
                        map_x = cmp::min(map_x, 1024);
                        map_y = cmp::min(map_y, 1024);
                    }
                },
                /* Up and down for vertical scroll. Not sure I'll keep this. */
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    map_y -= 8*zoom;
                    map_y = cmp::max(map_y,-1024);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    map_y += 8*zoom;
                    map_y = cmp::min(map_y,1024);
                },
                /* S is in the middle, so center ("senter"?) */
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    map_x = 0;
                    map_y = 0;
                },
                /* use Q and R for rotation. */
                Event::MouseWheel { x: -1, .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageUp), .. } => {
                    orientation = orientation.counterclockwise();
                    world_refresh_needed = true;
                },
                Event::MouseWheel { x: 1, .. } |
                Event::KeyDown { keycode: Some(Keycode::R), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => {
                    orientation = orientation.clockwise();
                    world_refresh_needed = true;
                },
                Event::MouseWheel { y: -1, .. } |
                Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
                    if zoom > 20 {
                        zoom -= 4;
                    } else if zoom > 12 {
                        zoom -= 2;
                    } else if zoom > 2 { // FIXME SHOULD BE 4
                        zoom -= 1;
                    }
                },
                Event::MouseWheel { y: 1, .. } |
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if zoom < 12 {
                        zoom += 1;
                    } else if zoom < 20 {
                        zoom += 2
                    } else if zoom < 29 {
                        zoom += 4;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x: mx, y: my, .. } => {
                    let click_point = Point::new((mx*1920)/draw_rect.width()  as i32 - draw_rect.x(),
                                                 (my*1080)/draw_rect.height() as i32 - draw_rect.y());
                    println!("Click ({},{})",click_point.x,click_point.y);
                    point_to_hex(click_point,map_x,map_y,zoom);
                    // FIXME: describe in TOML (see TODO)
                    if Rect::new(1664,968,256,96).contains_point(click_point) { // compass
                        orientation = orientation.clockwise();
                        world_refresh_needed = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x: mx, y: my,.. } => {
                    let click_point = Point::new((mx*1920)/draw_rect.width()  as i32 - draw_rect.x(),
                                                 (my*1080)/draw_rect.height() as i32 - draw_rect.y());
                    // FIXME: separate handling for all of the mouse clicks
                    // because this is gonna be so big.                        
                    //println!("{:?}",click_point);
                    // FIXME: describe in TOML (see TODO)
                    if Rect::new(1664,968,256,96).contains_point(click_point) { // compass                       
                        orientation = orientation.counterclockwise();
                        world_refresh_needed = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    islandmap.generate(64);
                    world_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    islandmap.generate_debug(16);
                    world_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    islandmap.generate_debug(32);
                    world_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    islandmap.generate_debug(64);
                    world_refresh_needed = true;
                },
                
                

                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    match canvas.window().fullscreen_state() {
                        video::FullscreenType::Off => { 
                            // avoid race condition with redraw window state change
                            if fullscreen_refresh_needed==0 {
                                canvas.window_mut().set_fullscreen(video::FullscreenType::Desktop).unwrap();
                            }
                        },
                        video::FullscreenType::Desktop => canvas.window_mut().set_fullscreen(video::FullscreenType::Off).unwrap(),
                        video::FullscreenType::True => unreachable!(),
                    };
                },
                Event::Window {win_event,..} => {
                    match win_event {
                        WindowEvent::Resized(wx,wy) => {
                            draw_rect = letterbox(wx,wy);
                            match canvas.window().fullscreen_state() {
                                video::FullscreenType::Off => {}, // if we allow resizing, snap?
                                video::FullscreenType::Desktop => fullscreen_refresh_needed=10,
                                video::FullscreenType::True => unreachable!(),
                            };
                        },
                        _ => { /* println!("{:?}",win_event); */ }
                    }
                    //println!("{:?}",win_event);
                
                },
                Event::MouseMotion{..} => { /* ignore */ },
                _ => { /* println!("{:?}",event); */ }
            }
        }
        
        // The rest of the game loop goes here...
        
        // Approximately 20fps        
        let next_tick = frame_ticker + time::Duration::from_millis(50);
        let now = time::Instant::now(); // fixme: better to call this only once per loop, but
        if now >= next_tick {
            if background_refresh_needed {
                canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
                    draw_background(texture_canvas, &sprite_atlas);
                }).unwrap();
                background_refresh_needed = false;
                //println!("Background Refresh     : {}",(time::Instant::now()-now).subsec_nanos()/1000000);
            }
            if world_refresh_needed {
                canvas.with_texture_canvas(&mut world_texture, |texture_canvas| {
                    draw_map(texture_canvas, &background_texture, &sprite_atlas, &islandmap, orientation);
                }).unwrap();
                world_refresh_needed = false;
                //println!("World Refresh Time: {}ms",(time::Instant::now()-now).subsec_nanos()/1000000);
            }

            if fullscreen_refresh_needed>0 {
                canvas.set_draw_color(Color::RGB(0,0,0));
                canvas.clear();
                fullscreen_refresh_needed -= 1;
            }

            let visible_w=1920/4*zoom;
            let visible_h=1080/4*zoom;
            let world_x = 16384/2-visible_w/2+((map_x*(16384-visible_w))/2048);  // 2048 is our scroll range
            let world_y = 8640/2 -visible_h/2+((map_y*(8640 -visible_h))/2048);

            canvas.copy(&world_texture,
                        Rect::new(world_x as i32, 
                                  world_y as i32,
                                  visible_w as u32,
                                  visible_h as u32),
                        draw_rect)
                        .expect("Render failed");
                        
                
            
            // FIXME -- move this to "ui overlay" function.
            {
                let (lw,lh)=canvas.logical_size();
                canvas.set_logical_size(1920,1080).unwrap();
                sprite_atlas.draw(canvas, "compass", 1, 1664, 968,orientation);    
                canvas.set_logical_size(lw,lh).unwrap();
            }
            
            canvas.present();
            
            // it's silly to do this here every time in the loop, but
            // somewhere around here (maybe _outside_ of the fps check here)
            // we will test for changing the cursor
            
            mouse_util.show_cursor(true);

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

