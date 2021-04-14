extern crate rand;
use rand::Rng;

pub fn ang(rand: &f32) -> &str {
    let mut outcome = "";
    if rand <= &26.00 {
        outcome = "LF";
    }
    else if rand <= &62.00 {
        outcome = "LC"
    }
    else if rand <= &82.00 {
        outcome = "CF"
    }
    else if rand <= &94.00 {
        outcome = "RC"
    }
    else if rand <= &100.00 {
        outcome = "RF"
    }
    {
        outcome
    }
}

pub fn ba(feild: &str) -> f32 {
    let mut angle = 00.00;
    if feild == "LF"{
        angle = rand::thread_rng().gen_range(0.00, 0.314159);
    }
    else if feild == "LC" {
        angle = rand::thread_rng().gen_range(0.314159, 0.628319);
    }
    else if feild == "CF" {
        angle = rand::thread_rng().gen_range(0.628319, 0.942478);
    }
    else if feild == "RC" {
        angle = rand::thread_rng().gen_range(0.942478, 1.25664);
    }
    else if feild == "RF" {
        angle = rand::thread_rng().gen_range(1.25664, 1.5708);
    }
    {
        angle
    }
}
