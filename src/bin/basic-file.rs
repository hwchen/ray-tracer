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
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2_f64;

            let ir = (255.9 * r) as i64;
            let ig = (255.9 * g) as i64;
            let ib = (255.9 * b) as i64;

            out.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }

    println!("{}", out);
}

