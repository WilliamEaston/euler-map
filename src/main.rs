use std::f64::consts::{FRAC_PI_4, PI};
use tiny_skia::*;
use gkquad::single::{Integrator, Points};

fn main() {
    let mercator = Pixmap::load_png(".\\wrld-21.png").unwrap();

    let size = 30000.0;
    let g = 0.05;
    let max_lat = 85.0;
    let dots_per_degree = 3.0;

    let t_upto = 360.0 * max_lat / g;
    let num_dots = (t_upto * dots_per_degree) as i32;

    let mut paint = Paint::default();
    let mut map = Pixmap::new((size * 2.0 / 1.9) as u32, (size * 2.0 / 1.9) as u32).unwrap();

    't_loop:
    for i in -num_dots..num_dots {
        let t = i as f64 / dots_per_degree;

        let (l, p) = line_to_long_lat(t, g);
        let (pix_x, pix_y) = pix_from_long_lat(l, p);

        // println!("t: {}, long: {}, lat: {}, pix: {:?}", t, l, p, (pix_x, pix_y));

        let option_pixel = mercator.pixel(pix_x, pix_y);
        let pixel = match option_pixel {
            Some(x) => x,
            None => {
                //println!("Continuing: ({}, {})", pix_x, pix_y);
                continue 't_loop;    // off the edge of the mercator map
            }
        };

        paint.set_color_rgba8(pixel.red(), pixel.green(), pixel.blue(), 255);

        let (spiral_x, spiral_y) = line_to_spiral(scale_t_for_spiral(t));
        let (output_x, output_y) = ((spiral_x * size/2.0 + size/1.9) as f32, (spiral_y * size/2.0 + size/1.9) as f32);

        map.fill_rect(Rect::from_xywh(output_x, output_y, 1.0, 1.0).unwrap(), &paint, Transform::identity(), None);
    }

    let _ = map.save_png(".\\output.png");
}

// extra +360) % 360 to get positive modulo result
fn line_to_long_lat(t: f64, g: f64) -> (f64, f64) {
    (((((t + 180.0) % 360.0) + 360.0) % 360.0) - 180.0,      t * g / 360.0)
}

fn line_to_spiral(t: f64) -> (f64, f64) {
    (Integrator::new(|s: f64| (s * s).cos())
        .max_iters(1000)
        .points(&*Points::new())
        .run(0.0..t)
        .estimate()
        .unwrap(),
    Integrator::new(|s: f64| (s * s).sin())
         .max_iters(1000)
         .points(&*Points::new())
         .run(0.0..t)
         .estimate()
         .unwrap())
}

// take better measurements
fn pix_from_long_lat(long: f64, lat: f64) -> (u32, u32) {
    ((7.8 * long + 1410.0) as u32 ,  (-457.0 * (FRAC_PI_4 + lat*PI / 360.0).tan().ln() + 1495.0) as u32)
}

fn scale_t_for_spiral(t: f64) -> f64 {
    ((2.0*PI*t.abs()) / 360.0).sqrt()  * t.signum()  // pull sign out of sqrt
}