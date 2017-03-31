extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;
const STARS_COUNT: u16 = 200;
const XY_RANGE: i32 = 25;
const MAX_DEPTH: u32 = 32;

struct Star {
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    fn new() -> Star {
        Star {
            x: xy_range(),
            y: xy_range(),
            z: rand::thread_rng().gen_range(1, MAX_DEPTH) as f32,
        }
    }
}

fn xy_range() -> f32 {
    rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f32
}

fn main() {
    let half_width = WIDTH / 2;
    let half_height = HEIGHT / 2;
    let mut stars: Vec<Star> = vec![];

    for i in 0..STARS_COUNT {
        stars.push(Star::new());
        println!("{} {} {}", stars[i as usize].x, stars[i as usize].y, stars[i as usize].z);
    }

    let mut window: PistonWindow =
        WindowSettings::new("Hello star.", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, WIDTH as f64, HEIGHT as f64],
                      c.transform, g);
            for star in &mut stars {
                star.z -= 0.2;
                if( star.z <= 0.0 ) {
                    star.x = xy_range();
                    star.y = xy_range();
                    star.z = MAX_DEPTH as f32;
                }
                let k  = 128.0 / star.z;
                let px = star.x * k + half_width;
                let py = star.y * k + half_height;

                if( px >= 0.0 && px <= 500.0 && py >= 0.0 && py <= 400.0 ) {
                    let size = (1.0 - star.z / 32.0) * 5.0;
                    let shade = (1.0 - star.z / 32.0);
                    rectangle([0.0, 0.0, 0.0, 1.0],
                      [px, py, size, size],
                      c.transform, g);

                    ctx.fillStyle = "rgb(" + shade + "," + shade + "," + shade + ")";
                    ctx.fillRect(px,py,size,size);
                }
            }
        });
    }
}