use macroquad::prelude::*;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Copy, Clone)]
struct ComplexNumber {
    r: f64,
    i: f64,
}

const MAX_ITERATIONS: i32 = 1_000;
const CUT_OFF_BOUND: f64 = 2.;
impl std::ops::Mul<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: ComplexNumber) -> ComplexNumber {
        return ComplexNumber {
            r: self.r*other.r - self.i*other.i,
            i: self.r*other.i + self.i*other.r
        }
    }
}
impl std::ops::Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        return ComplexNumber {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}
impl ComplexNumber {
    fn new(x:f64, y: f64) -> Self {
        Self {
            r: x,
            i: y
        }
    }
    fn abs(&self) -> f64 {
        f64::hypot(self.r, self.i)
    }

    fn julia_iteration(&mut self, c: ComplexNumber) {
        *self = *self * *self + c;
    }

    fn compute_iterations(mut self, c: ComplexNumber) -> i32 {
        for i in 0..MAX_ITERATIONS {
            self.julia_iteration(c);
            if self.abs() > CUT_OFF_BOUND {
                return i;
            }
        }
        return MAX_ITERATIONS;
    }
}

fn julia_color(num_iterations : i32) -> Color {
    let intensity = num_iterations as f32 / MAX_ITERATIONS as f32;
    
    Color {
        r: intensity,
        g: intensity,
        b: intensity,
        a: 1.,
    }
}

fn draw_julia(x_shift: f64, y_shift: f64, zoom: f64, c: ComplexNumber) {
    clear_background(BLACK);
    let (tx, rx) = mpsc::channel();
    for y in 0..(screen_height().ceil() as i32) {
        let tx = tx.clone();
        thread::spawn(move || {
            let val = draw_julia_row(x_shift, y_shift, zoom, c, y);
            tx.send(val).unwrap();
        });
    }
    for _ in 0..(screen_height().ceil() as i32) {
        let (y, colors) = rx.recv().unwrap();
        for (x, color) in colors.iter().enumerate() {
            draw_rectangle(x as f32, y as f32, 1., 1., *color);
        }
    }

    
}
fn draw_julia_row(x_shift: f64, y_shift: f64, zoom: f64, c: ComplexNumber, y: i32) -> (i32, Vec<Color>) {
    let mut colors = Vec::new();
    for x in 0..(screen_width().ceil() as i32) {
        let point = ComplexNumber::new((x as f64 - x_shift) / zoom, -(y as f64 - y_shift)/ zoom);
        let num_iter = point.compute_iterations(c);
        colors.push(julia_color(num_iter));
    }
    return (y, colors);
}
fn change_zoom(zoom: &mut f64) -> bool {
    // Zooming with ctrl+/ctrl-
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Minus) {
        *zoom /= 1.5;
        return true;
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Equal) {
        *zoom *= 1.5;
        return true;
    }

    false
}

fn change_center(x_shift: &mut f64, y_shift: &mut f64) -> bool {
    let mut moved = false;
    let movements = [(KeyCode::Left, 1., 0.), (KeyCode::Right, -1., 0.), (KeyCode::Up, 0., 1.), (KeyCode::Down, 0., -1.)];
    for (key, dx, dy) in &movements {
        if is_key_down(*key) {
            *x_shift += dx * 10.;
            *y_shift += dy * 10.;
            moved = true;
        }
    }
    return moved;

}

fn change_c(c: &mut ComplexNumber) {
    let (dx, dy) = (0.001, 0.001); 
    if is_key_down(KeyCode::W) {
        c.i += dy;
    }
    if is_key_down(KeyCode::A) {
        c.r -= dx;
    }
    if is_key_down(KeyCode::S) {
        c.i -= dy;
    }
    if is_key_down(KeyCode::D) {
        c.r += dx;
    }
}

#[macroquad::main("Julia Sets :)")]
async fn main() {
    let mut zoom = 250.;
    let mut c = ComplexNumber::new(-0.391, -0.587);
    let mut x_shift =  screen_width().ceil() as f64 / 2.;
    let mut y_shift =  screen_height().ceil() as f64 / 2.;
    loop {
        println!("{}", get_frame_time());
        change_zoom(&mut zoom);
        change_center(&mut x_shift, &mut y_shift);
        change_c(&mut c);
        draw_julia(x_shift.clone(), y_shift.clone(), zoom.clone(), c.clone());
        next_frame().await
    }
}
