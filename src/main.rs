extern crate piston_window;
extern crate rand;
mod bubble;

use piston_window::*;
use bubble::Bubble;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 500;
const BG_COLOR: [f32; 4] = [245.0/255.0, 177.0/255.0, 88.0/255.0, 1.0];
const INTERPOLATION_STEPS: u16 = 50;

fn main() {
    
    let mut window: PistonWindow = WindowSettings::new("Lava lamp", Size::from((WIDTH, HEIGHT)))
        .build()
        .unwrap();
    let mut bubbles: Vec<Bubble> = generate_bubbles(100);
    let mut events = window.events.max_fps(30);

    let mut color_of_bubbles: [f32; 4] = generate_random_color();
    let mut target_color_of_bubbles: [f32; 4] = generate_random_color();

    let mut interpolation_step_r: f32 = (target_color_of_bubbles[0] - color_of_bubbles[0]) / INTERPOLATION_STEPS as f32;
    let mut interpolation_step_g: f32 = (target_color_of_bubbles[1] - color_of_bubbles[1]) / INTERPOLATION_STEPS as f32;
    let mut interpolation_step_b: f32 = (target_color_of_bubbles[2] - color_of_bubbles[2]) / INTERPOLATION_STEPS as f32;
    let mut interpolation_step_alpha: f32 = (target_color_of_bubbles[3] - color_of_bubbles[3]) / INTERPOLATION_STEPS as f32;


    while let Some(e) = events.next(&mut window) {
            
            // render
            if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g, _| { 
                    let bubs = &bubbles;

                    color_of_bubbles[0] += interpolation_step_r;
                    color_of_bubbles[1] += interpolation_step_g;
                    color_of_bubbles[2] += interpolation_step_b;
                    color_of_bubbles[3] += interpolation_step_alpha;

                    clear(BG_COLOR, g);
                    for b in bubs.iter() {
                        ellipse(
                            color_of_bubbles,
                            [b.x - b.r1, b.y - b.r2, b.r1 * 2.0, b.r2 * 2.0], 
                            c.transform, 
                            g
                        );
                    }

                    if (target_color_of_bubbles[0] - color_of_bubbles[0]).abs() < 0.01 && 
                        (target_color_of_bubbles[1] - color_of_bubbles[1]).abs() < 0.01 && 
                        (target_color_of_bubbles[2] - color_of_bubbles[2]).abs() < 0.01 && 
                        (target_color_of_bubbles[3] - color_of_bubbles[3]).abs() < 0.01 
                    {
                        target_color_of_bubbles = generate_random_color();
                        interpolation_step_r = (target_color_of_bubbles[0] - color_of_bubbles[0]) / INTERPOLATION_STEPS as f32;
                        interpolation_step_g = (target_color_of_bubbles[1] - color_of_bubbles[1]) / INTERPOLATION_STEPS as f32;
                        interpolation_step_b = (target_color_of_bubbles[2] - color_of_bubbles[2]) / INTERPOLATION_STEPS as f32;
                        interpolation_step_alpha = (target_color_of_bubbles[3] - color_of_bubbles[3]) / INTERPOLATION_STEPS as f32;
                    }
                });
            };
            // update
            if let Some(_) = e.update_args() {
                let bubs = &mut bubbles;
                for b in bubs.iter_mut() {
                    b.update();
                }
            }
    }
}

pub fn generate_random_color() -> [f32; 4] {
    [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]
}

pub fn generate_bubbles(ammount: i32) -> Vec<Bubble> {
    let mut bubbles: Vec<Bubble> = Vec::new();
    for _ in 0..ammount {
        bubbles.push(Bubble::new(
            rand::random::<f64>() * WIDTH as f64,
            rand::random::<f64>() * HEIGHT as f64,
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>()
        ));
    }
    bubbles
}
