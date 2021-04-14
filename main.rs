mod at_bats;
mod pitch_sim;
mod distance;
mod angle;
mod outcome;
extern crate rand;
use rand::Rng;
use fltk::*;

fn main() {
    
    let season_abs = at_bats::run();
    let mut hits = vec!{};
    let mut strikeouts = 0;
    let mut outs = 0;
    let mut singles = 0;
    let mut doubles = 0;
    let mut triples = 0;
    let mut homeruns = 0;
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
                let hard_hit = rand::thread_rng().gen_range(0, 1000);
                let a = pitch_sim::good_hit(&hard_hit);
                let an = rand::thread_rng().gen_range(0.00, 100.00);
                let feild = angle::ang(&an);
                let angle = angle::ba(&feild);
                let la;
                let ev;
                let sh;
                let dist;
                let hit;
                if a == true{
                    la = rand::thread_rng().gen_range(-0.261799, 0.493599);
                    ev = rand::thread_rng().gen_range(133.467, 153.50667);
                    sh = rand::thread_rng().gen_range(1.641667, 3.525);
                    dist = distance::dist(&la, &ev, &sh);
                    }
                    else {
                    let pop_ground = rand::thread_rng().gen_weighted_bool(2);
                        if pop_ground == true {
                            la = rand::thread_rng().gen_range(0.523599, 1.0472);
                            ev = rand::thread_rng().gen_range(73.3333, 133.467);
                            sh = rand::thread_rng().gen_range(1.641667, 3.525); 
                            dist = distance::dist(&la, &ev, &sh);
                        }
                        else {
                            la = rand::thread_rng().gen_range(-0.855211, -0.174533);
                            ev = rand::thread_rng().gen_range(73.3333, 153.467);
                            sh = rand::thread_rng().gen_range(1.641667, 3.525); 
                            dist = distance::dist(&la, &ev, &sh);
                        }  
                        
                    }
                    hit = outcome::hit(&ev, &an, &dist, &feild);
                    if hit != "out" {
                        let tuple = (hit, dist, angle);
                        hits.push(tuple);
                        if hit == "single" {
                            singles+=1;
                        }
                        if hit == "double" {
                            doubles+=1;
                        }
                        if hit == "triple" {
                            triples+=1;
                        }
                        if hit == "HR" {
                            homeruns+=1;
                        }
                    }
                    else {
                        outs += 1;
                    }
                break;
            }
            else {
                strikes += 1;
            }
            j += 1;
            if strikes == 3 {
                strikeouts += 1;
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
    let tuple1 = ("HR", 396.00, 0.471239);
    hits.push(tuple1);
    let tuple2 = ("HR", 432.00, 1.09956);
    hits.push(tuple2);
    let tuple3 = ("HR", 406.00, 0.959931);
    hits.push(tuple3);
    let tuple4 = ("single", 175.00, 0.174533);
    hits.push(tuple4);
    let tuple5 = ("HR", 384.00, 0.0698132);
    hits.push(tuple5);
    let tuple6 = ("HR", 428.00, 0.366519);
    hits.push(tuple6);
    let tuple7 = ("HR", 368.00, 0.139626);
    hits.push(tuple7);
    let tuple8 = ("single", 59.00, 0.820305);
    hits.push(tuple8);
    let tuple9 = ("HR", 366.00, 0.10472);
    hits.push(tuple9);
    homeruns +=7;
    singles +=2;
    outs +=23;
    for tuple in hits.iter() {
        println!("Outcome: {}", tuple.0);
        println!("Distance: {}", tuple.1);
        println!("Angle: {}", tuple.2);
    }
    println!("strikeouts: {}", strikeouts);
    println!("outs: {}", outs);
    println!("singles: {}", singles);
    println!("doubles: {}", doubles);
    println!("triples: {}", triples);
    println!("Homeruns: {}", homeruns);
    let avg = (hits.len() as f32) / (239 as f32);
    println!("Avg: {:.3}", avg);
    let slg = (singles + (2*doubles) * (3*triples) + (4*homeruns)) as f32 / (239 as f32);
    println!("SLG: {:.3}", slg);
    let ops = avg + slg;
    println!("OBS: {:.3}", ops);
    let ks = strikeouts.to_string();
    let avg2 = avg.to_string();
    let slg2 = slg.to_string();
    let ops2 = ops.to_string();
    let sig2 = singles.to_string();
    let dub2 = doubles.to_string();
    let tip2 = triples.to_string();
    let hr2 = homeruns.to_string();



    let app = app::App::default();
    let mut wind = fltk::window::Window::new(100, 100, 800, 800, "Pete Alonso's 2020 spray chart");
    let mut frm = fltk::frame::Frame::new(0,0,800,800, "");
    wind.end();
    wind.show();

    wind.set_color(Color::White);
    frm.draw(Box::new(move || {
        draw::set_draw_color(Color::Black);
        draw::draw_line(400, 700, 633, 467);
        draw::draw_line(400, 700, 167, 467);
        draw::draw_line(195, 407, 167, 467);
        draw::draw_line(195, 407, 346, 292);
        draw::draw_line(454, 292, 346, 292);
        draw::draw_line(454, 292, 541, 359);
        draw::draw_line(633, 467, 541, 359);
        draw::draw_line(464, 636, 400, 572);
        draw::draw_line(336, 636, 400, 572);

        draw::draw_circle(400.00, 700.00, 170.00);
        draw::draw_rect_fill(0, 580, 280, 580, Color::White);
        draw::draw_rect_fill(521, 580, 180, 220, Color::White);
        
        draw::set_draw_color(Color::Black);
        draw::draw_circle(400.00, 636.00, 10.00);
        draw::draw_rect(395, 635, 10, 2);

        draw::draw_text("Strikeouts:", 20, 680);
        draw::draw_text(&ks, 90, 680);
       
        draw::draw_text("Singles:", 20, 700);
        draw::draw_text(&sig2, 70, 700);
        draw::draw_rect_fill(89, 693, 5, 5, Color::Blue);
        draw::set_draw_color(Color::Black);

        draw::draw_text("Doubles:", 20, 720);
        draw::draw_text(&dub2, 75, 720);
        draw::draw_rect_fill(90, 713, 5, 5, Color::Cyan);
        draw::set_draw_color(Color::Black);

        draw::draw_text("Triples:", 20, 740);
        draw::draw_text(&tip2, 67, 740);
        draw::draw_rect_fill(85, 733, 5, 5, Color::Black);

        draw::draw_text("Homeruns:", 20, 760);
        draw::draw_text(&hr2, 90, 760);
        draw::draw_rect_fill(109, 753, 5, 5, Color::Red);
        draw::set_draw_color(Color::Black);

        draw::draw_text("Average:", 500, 700);
        draw::draw_text(&avg2, 560, 700);

        draw::draw_text("Slugging percentage:", 500, 720);
        draw::draw_text(&slg2, 640, 720);

        draw::draw_text("OPS:", 500, 740);
        draw::draw_text(&ops2, 535, 740);
        
       
        for tuple in hits.iter() {
            if tuple.0 == "single" {
                draw::set_draw_color(Color::Blue);
            }
            else if tuple.0 == "double" {
                draw::set_draw_color(Color::Cyan);
            }
            else if tuple.0 == "triple" {
                draw::set_draw_color(Color::Black);
            }
            else {
                draw::set_draw_color(Color::Red);
            }
            let mut angle = tuple.2;
            let angle2 = tuple.2;
            if angle < 0.785398 {
                angle = angle + 0.785398;
            } 
            else {
                angle = 3.14159 - (0.785398 + angle);
            }
            let dist = tuple.1;
            let mut y1 = dist * angle.sin();
            let mut x1 = (dist*dist) - (y1*y1);
            x1 = x1.sqrt();
            y1 = 700.00-y1;
            if angle2 < 0.785398 {
                x1 = 400.00 - x1;

                draw::draw_rect_fill(x1 as i32, y1 as i32, 5, 5, draw::get_color());
            } 
            else {
                x1 = 400.00 + x1;
                draw::draw_rect_fill(x1 as i32, y1 as i32, 5, 5, draw::get_color());
            }
        }
         
    }));
    app.run().unwrap();
}
