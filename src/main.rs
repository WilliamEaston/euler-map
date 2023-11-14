use std::f64::consts::{FRAC_PI_4, PI};
use std::thread;
use tiny_skia::*;
use gkquad::single::{Integrator, Points};

fn main() {
    let size = 1000000.0;
    let focused_area_size = 3000.0;
    let g = 0.001;
    let max_lat = 85.0;
    let dots_per_degree = 15.0;

    let t_upto = 360.0 * max_lat / g;
    let num_dots = (t_upto * dots_per_degree) as i32;
    let spiral_center = (PI/8.0).sqrt();

    let num_threads = 5;
    let mut handles = Vec::with_capacity(num_dots as usize);
    let dots_per_thread = num_dots / num_threads;

    for j in 0..num_threads {
        handles.push(thread::spawn(move || {

            let mercator =  Pixmap::load_png(".\\wrld-21.png").unwrap();
            let mut points: Vec<(f32, f32, u8, u8, u8)> = vec![];
            let mut integral_cache = IntegralCache{upto: 0.0, result: (0.0, 0.0)};

            't_loop:
            for i in dots_per_thread*j..dots_per_thread*(j+1) {
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

                let (spiral_x, spiral_y) = line_to_spiral(scale_t_for_spiral(t), &mut integral_cache);
                let (output_x, output_y) = ((spiral_x * size/2.0 + size/2.0) as f32, (spiral_y * size/2.0 + size/2.0) as f32);

                let (zoomed_x, zoomed_y) = get_zoomed_coords(output_x, output_y, size, focused_area_size, spiral_center);

                points.push((zoomed_x, zoomed_y, pixel.red(), pixel.green(), pixel.blue()));
            }
            points
        }));
    }

    let mut map = Pixmap::new(focused_area_size as u32, focused_area_size as u32).unwrap();
    let mut paint = Paint::default();

    for handle in handles {
        for point in handle.join().unwrap() {
            paint.set_color_rgba8(point.2, point.3, point.4, 255);
            map.fill_rect(Rect::from_xywh(point.0, point.1, 1.0, 1.0).unwrap(), &paint, Transform::identity(), None);
        }
    }

    let _ = map.save_png(".\\output.png");
}

// extra +360) % 360 to get positive modulo result
fn line_to_long_lat(t: f64, g: f64) -> (f64, f64) {
    (((((t + 180.0) % 360.0) + 360.0) % 360.0) - 180.0,      t * g / 360.0)
}

fn line_to_spiral(t: f64, cache: &mut IntegralCache) -> (f64, f64) {
    let out = (Integrator::new(|s: f64| (s * s).cos())
        .max_iters(200000000)
        .points(&*Points::new())
        .run(cache.upto..t)
        .estimate()
        .unwrap() + cache.result.0,
    Integrator::new(|s: f64| (s * s).sin())
         .max_iters(200000000)
         .points(&*Points::new())
         .run(cache.upto..t)
         .estimate()
         .unwrap() + cache.result.1);

    cache.upto = t;
    cache.result = out;

    out
}

// take better measurements
fn pix_from_long_lat(long: f64, lat: f64) -> (u32, u32) {
    ((7.8 * long + 1410.0) as u32 ,  (-457.0 * (FRAC_PI_4 + lat*PI / 360.0).tan().ln() + 1495.0) as u32)
}

fn scale_t_for_spiral(t: f64) -> f64 {
    ((2.0*PI*t.abs()) / 360.0).sqrt()  * t.signum()  // pull sign out of sqrt
}

fn get_zoomed_coords(x: f32, y: f32, overall_size: f64, zoomed_size: f64, center: f64) -> (f32, f32) {
    let spiral_center = (overall_size / 2.0) * center + (overall_size / 2.0);
    let zoomed_diff = (-spiral_center + zoomed_size / 2.0) as f32;

    (x + zoomed_diff, y + zoomed_diff)
}

struct IntegralCache {
    upto: f64,
    result: (f64, f64)
}