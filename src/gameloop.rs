extern crate sdl2;

// The event loop handles... events -- and also basic drawing.


use sdl2::event::Event;
use sdl2::keyboard::Keycode;



use std::time;
use std::thread;

pub fn gameloop(canvas: &mut sdl2::render::WindowCanvas, event_pump: &mut sdl2::EventPump) {

    canvas.clear();
    canvas.present();

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

        canvas.present();

        thread::sleep(time::Duration::from_millis(10));
    }
}
