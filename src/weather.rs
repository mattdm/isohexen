extern crate rand;
use rand::Rng;

use std::time;
use std::thread;

use std::sync::mpsc;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Cloud {
    pub size: i32,
    pub position: i32,
    pub altitude: i32,
}

pub fn cloud_controller(tx: mpsc::SyncSender<Vec<Cloud>>) {
    let mut rng = rand::thread_rng();

    let mut cloud_ticker = time::Instant::now();        

    let mut cloudlist = Vec::new();
    //start with 0, 1, 2, or 3 clouds
    for _ in 0..(rng.gen::<u32>()%4) {
        cloudlist.push( Cloud {
                                size: 2,
                                position: (rng.gen::<u32>()%(16384+256*6)) as i32-256*3,
                                altitude: (rng.gen::<u32>()%768) as i32,
                              });
    }                            
    
    loop {


        let mut nextclouds = Vec::new();    

        for cloud in &cloudlist {
            let new_pos = cloud.position - (cloud.altitude/192+1); // fix -- adjustable speeds
            if new_pos > -256*3 {
                // still on screen
                        
                nextclouds.push( Cloud {
                                          size: 2,
                                          position: new_pos,
                                          altitude: cloud.altitude,
                                       });
                
            }
        };
        
        // small chance of a new cloud, scaled by existing number of clouds
        // FIXME: the magic weight numbers are an easy way to adjust the weather
        if rng.gen_weighted_bool((20 as i32).pow(nextclouds.len() as u32) as u32 + 10000) {
            nextclouds.push( Cloud {
                                     size: 2,
                                     position: 16384,
                                     altitude: (rng.gen::<u32>()%768) as i32,
                                   });
        }
        
        cloudlist = nextclouds.clone();
        match tx.send(nextclouds) {
            _ => {}, // we super, super, super-duper don't care if this fails :)	
        }

        let next_tick = cloud_ticker + time::Duration::from_millis(1000);
        let now = time::Instant::now();
        if now < next_tick {
            thread::sleep(next_tick-now);
        }
        cloud_ticker = next_tick;
    }    
}
