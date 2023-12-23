extern crate rand;

use rand::Rng;
use std::env;

fn cost(t: [[f32; 2]; 5], w: f32, b: f32) -> f32 {
    let mut r: f32 = 0.0;
    for pair in t {
        let val = pair[0];
        let exp = pair[1];
        let act = val * w + b;
        let dif = act - exp;
        r = r + (dif * dif);
    }
    return r / t.len() as f32;
}

fn main() {
    let t: [[f32; 2]; 5] = [[0.0, 0.0], [1.0, 2.0], [2.0, 4.0], [3.0, 6.0], [4.0, 8.0]];

    let mut r: u32 = 5;
    let tmp: Vec<String> = env::args().collect();
    if tmp.len() > 1 {
        r = tmp[1].parse().unwrap();
    }

    let mut rng = rand::thread_rng();
    let mut w: f32 = rng.gen::<f32>() * 10.0;
    let mut b: f32 = rng.gen::<f32>() * 5.0;
    let eps: f32 = 1e-3;
    let rate: f32 = 1e-3;

    for _i in 0..r {
        let c = cost(t, w, b);
        let dcostw = (cost(t, w + eps, b) - c) / eps;
        let dcostb = (cost(t, w, b + eps) - c) / eps;
        w = w - rate * dcostw;
        b = b - rate * dcostb;
        /* println!("{} {} {} {} {}", c, dcostw, dcostb, w, b); */
    }

    for p in t {
        let input = p[0];
        let guess = w.ceil();
        let correct = p[1];
        println!("{} * {} = {} ({})", input, guess, (input * guess), correct);
    }
}
