extern crate rand;
use rand::Rng;

use std::time;
use std::thread;
use std::cmp;
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
    let mut speed_control = 0;

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
            // fix -- adjustable wind speeds

            // lower clouds move more slowly
            if speed_control % ((768-cloud.altitude)/256+1) == 0 {
                let new_pos = cloud.position - 1;
                if new_pos > -256*3 {
                    // still on screen
                    nextclouds.push( Cloud {
                                              size: 2,
                                              position: new_pos,
                                              altitude: cloud.altitude,
                                           });
                }
            } else {
                nextclouds.push(cloud.clone());
            }
                
        }
        
        // small chance of a new cloud, scaled by existing number of clouds
        // FIXME: the magic weight numbers are an easy way to adjust the weather
        if rng.gen_weighted_bool(200) &&
           rng.gen_weighted_bool((4 as i32).pow(cmp::min(15,nextclouds.len() as u32)) as u32) {
           nextclouds.push( Cloud {
                                     size: 2,
                                     position: 16384,
                                     altitude: (rng.gen::<u32>()%768) as i32,
                                   });
        }


        //println!("Clouds: {}",nextclouds.len());

        nextclouds.sort_unstable_by_key(|k| -k.altitude);
        cloudlist = nextclouds.clone();


        match tx.send(nextclouds) {
            _ => {}, // we super, super, super-duper don't care if this fails :)	
        }

        let next_tick = cloud_ticker + time::Duration::from_millis(200);
        let now = time::Instant::now();
        if now < next_tick {
            thread::sleep(next_tick-now);
        }
        cloud_ticker = next_tick;
        speed_control += 1;
        if speed_control >= 12 {
            speed_control = 0;
        }
    }    
}
