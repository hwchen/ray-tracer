extern crate ray_tracer;

use ray_tracer::{Vec3, Point, Color};

fn main() {
    let nx = 200;
    let ny = 100;
    let mut out = "P3\n".to_owned();

    out.push_str(&nx.to_string());
    out.push_str(" ");
    out.push_str(&ny.to_string());
    out.push_str("\n255\n");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color_vec = Color::new(
                i as f64 / nx as f64,
                j as f64 / ny as f64,
                0.2_f64,
            );

            let i_rgb = color_vec * 255.9; // allocates new

            out.push_str(&format!("{}\n", i_rgb.to_ppm_tuple_int()));
        }
    }

    println!("{}", out);
}


