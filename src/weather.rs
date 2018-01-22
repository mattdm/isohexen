extern crate rand;
use rand::Rng;

use std::time;
use std::thread;

use std::sync::mpsc;



pub fn cloud_controller(tx: mpsc::SyncSender<(i32,i32,i32)>) {
    let mut rng = rand::thread_rng();

    let mut cloud_ticker = time::Instant::now();        

    let mut cloud1 = (rng.gen::<u32>()%(16384+256*6)) as i32-256*3;
    let mut cloud2 = (rng.gen::<u32>()%(16384+256*6)) as i32-256*3;
    let mut cloud3 = (rng.gen::<u32>()%(16384+256*6)) as i32-256*3;
    
    loop {

    
        match tx.send((cloud1,cloud2,cloud3)) {
            _ => {}, // we super, super, super-duper don't care if this fails :)	
        }
        //tx.send((cloud1,cloud2,cloud3)).unwrap();

        let next_tick = cloud_ticker + time::Duration::from_millis(500);
        cloud1 -= 1;
        if cloud1 < -256*3 {
            cloud1 = 16384+256*3;
        }
        cloud2 -= 2;
        if cloud3 < -256*3 {
            cloud3 = 16384+256*3;
        }
        cloud3 -= 3;
        if cloud3 < -256*3 {
            cloud3 = 16384+256*3;
        }

        let now = time::Instant::now();
        if now < next_tick {
            thread::sleep(next_tick-now);
        }
        cloud_ticker = next_tick;
    }    
}
