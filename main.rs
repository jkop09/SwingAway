mod at_bats;
mod ab_sim;
mod pitch_sim;
mod distance;
extern crate rand;
use rand::Rng;

fn main() {
    
    let season_abs = at_bats::run();
    let mut hits = 0;
    let mut strikeouts = 0;
    let mut x = 0;
    while x < season_abs.len() {
        let mut j = 0;
        let y = &season_abs[x];
        let mut strikes = 0;
        while j < y.len() + 1 {
            let hit_p = rand::thread_rng().gen_range(0, 10000);
            let i = &y[j];
            let h = pitch_sim::run(&i, &hit_p);
            if h == true {
                hits += 1;
                println!("hit");
                let hard_hit = rand::thread_rng().gen_range(0, 1000);
                let a = pitch_sim::good_hit(&hard_hit);
                if a == true{
                    let la = rand::thread_rng().gen_range(-0.261799, 0.523599);
                    let ev = rand::thread_rng().gen_range(133.467, 173.50667);
                    let sh = rand::thread_rng().gen_range(1.641667, 3.525);
                    let dist = distance::dist(&la, &ev, &sh);
                    println!("Distance of hit: {}", dist);
                    }
                    else {
                    let pop_ground = rand::thread_rng().gen_weighted_bool(2);
                        if pop_ground == true {
                            let la = rand::thread_rng().gen_range(0.523599, 1.0472);
                            let ev = rand::thread_rng().gen_range(73.3333, 133.467);
                            let sh = rand::thread_rng().gen_range(1.641667, 3.525); 
                            let dist = distance::dist(&la, &ev, &sh);
                            println!("Distance of hit: {}", dist); 
                        }
                        else {
                        let la = rand::thread_rng().gen_range(-0.855211, -0.174533);
                        let ev = rand::thread_rng().gen_range(73.3333, 133.467);
                        let sh = rand::thread_rng().gen_range(1.641667, 3.525); 
                        let dist = distance::dist(&la, &ev, &sh);
                        println!("Distance of hit: {}", dist);
                        }  
                    }
                break;
            }
            else {
                strikes += 1;
            }
            j += 1;
            if strikes == 3 {
                strikeouts += 1;
                println!("strikeout");
                break;
            }
            if j == y.len() {
                j = 0;
            }
        }
        x += 1;
        if x > season_abs.len(){
            break;
        }
    }
    println!("hits: {}", hits);
    println!("strikeouts: {}", strikeouts);
}
