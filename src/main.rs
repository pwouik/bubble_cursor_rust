use std::mem;

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
#[macroquad::main("Bubble")]
async fn main() {
    request_new_screen_size(1000.0,600.0);
    let mut rng = thread_rng();
    let mut circles:Vec<(Vec2,f32)>=Vec::new();
    for _ in 0..30{
        circles.push((vec2(rng.gen_range(0.0f32..1000.0),rng.gen_range(0.0f32..600.0)),rng.gen_range(5.0..50.0)));
    }
    loop {
        clear_background(BLACK);
        let cur = Vec2::from(mouse_position());
        let mut min_dist = f32::MAX;
        let mut min_dist2 = f32::MAX;
        let mut closest = 0usize;
        for (i,circle) in circles.iter().enumerate(){
            let dist = circle.0.distance(cur)-circle.1;
            min_dist2 = min_dist2.min(dist);
            if min_dist2<min_dist {
                mem::swap(&mut min_dist, &mut min_dist2);
                closest = i;
            }
        }

        draw_circle(cur.x, cur.y, min_dist2.min(min_dist+2.0*circles[closest].1)+3.0, RED);
        draw_circle(circles[closest].0.x, circles[closest].0.y, circles[closest].1+3.0, RED);
        circles.iter().for_each(|(pos,width)| draw_circle(pos.x, pos.y, *width, WHITE));
        next_frame().await
    }
}