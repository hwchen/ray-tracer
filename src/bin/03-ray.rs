extern crate ray_tracer;

use ray_tracer::ray::Ray;
use ray_tracer::vec3::{Vec3, Point, Color};

fn main() {
    let nx = 200;
    let ny = 100;
    let mut out = "P3\n".to_owned();

    out.push_str(&nx.to_string());
    out.push_str(" ");
    out.push_str(&ny.to_string());
    out.push_str("\n255\n");

    let lower_left_corner = Point::new(-2.0, -1.0, -1.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 2.0, 0.0);
    let origin = Point::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let ray = Ray::new(
                // TODO remove clones!
                origin.clone(),
                lower_left_corner.clone() + horizontal.clone()*u + vertical.clone()*v,
            );

            let col = color_gradient(ray);

            let i_rgb = col * 255.9; // allocates new

            out.push_str(&format!("{}\n", i_rgb.to_ppm_tuple_int()));
        }
    }

    println!("{}", out);
}

fn color_gradient(ray: Ray) -> Color {
    let unit_direction = ray.direction.into_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

