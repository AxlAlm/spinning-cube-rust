use crate::math::*;

const CUBE_WIDTH: usize = 20.0;
const HORIZONTL_OFFSET: usize = -2 * CUBE_WIDTH;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const K1: usize = 40;

const X: usize = 0;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn calculate_x(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.sin() * angle.y.sin() * angle.z.cos()
        - pos.z * angle.x.cos() * angle.y.sin() * angle.z.cos()
        + pos.y * angle.x.cos() * angle.z.sin()
        + pos.z * angle.x.sin() * angle.z.sin()
        + pos.x * angle.y.cos() * angle.z.cos();
}

fn calculate_y(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.cos() * angle.z.cos() + pos.z * angle.x.sin() * angle.z.cos()
        - pos.y * angle.x.sin() * angle.y.sin() * angle.z.sin()
        + pos.z * angle.x.cos() * angle.y.sin() * angle.z.sin()
        - pos.x * angle.y.cos() * angle.z.sin();
}

fn calculate_z(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.z * angle.x.cos() * angle.y.cos() - pos.y * angle.x.sin() * angle.y.cos()
        + pos.x * angle.y.sin();
}

fn get_idx_to_update(pos: &Vec3, angle: &Vec3) -> (usize, f32) {
    let x = calculate_x(&pos, &angle);
    let y = calculate_y(&pos, &angle);
    let z = calculate_z(&pos, &angle) + 1.0;
    // let ooz = 1 / z;
    let xp = WIDTH / 2 + HORIZONTL_OFFSET + K1 * ooz * x * 2;
    let yp = HEIGHT / 2 + K1 * ooz * y;
    return (xp + yp * WIDTH, ooz);
}

//z_buffer: Vec<f32>,
fn update_surface(idx: usize, buffer: Vec<u8>, ch: &u8) {
    if idx < (&WIDTH * &HEIGHT) {
        buffer[idx] = *ch;

        // if ooz > z_buffer[idx] {
        //     z_buffer[idx] = ooz;
        //     buffer[idx] = *ch;
        // }
    }
}

fn calculate_for_surface(pos: &Vec3, angle: &Vec3, ch: &u8) -> usize {
    let idx = get_idx_to_update(&pos, &angle);
}

fn render(buffer: Vec<u8>) {
    print!("{}[1;1H", 27 as char);
    for k in 0..&WIDTH * &HEIGHT {
        if k % &WIDTH == 0 {
            println!();
        } else {
            print!("{}", *buffer.get(k).unwrap() as char)
        }
    }
}

fn main() {
    loop {
        for cube_x in -20..20 {
            for cube_y in -20..20 {}
        }
    }
}