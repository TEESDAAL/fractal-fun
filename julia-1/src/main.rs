use macroquad::prelude::*;
use std::sync::mpsc;
use std::thread;
use colorgrad;


const MAX_ITERATIONS: i32 = 1_000;
const CUT_OFF_BOUND: f32 = 2.;
const CUT_OFF_BOUND_SQUARED: f32 = CUT_OFF_BOUND * CUT_OFF_BOUND;
const ANTI_ALIASING_POINTS: i32 = 0;


#[derive(Debug, Copy, Clone)]
struct ComplexNumber {
    r: f32,
    i: f32,
}

impl std::ops::Mul<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: ComplexNumber) -> ComplexNumber {
        return ComplexNumber {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        };
    }
}

impl std::ops::Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        return ComplexNumber {
            r: self.r + other.r,
            i: self.i + other.i,
        };
    }
}

impl std::ops::Div<f32> for ComplexNumber {
    type Output = ComplexNumber;
    
    fn div(self, other: f32) -> ComplexNumber {
        ComplexNumber {
            r: self.r / other,
            i: self.i / other,
        }
    }
}

impl ComplexNumber {
    fn new(x: f32, y: f32) -> Self {
        Self { r: x, i: y }
    }

    fn abs(&self) -> f32 {
        f32::hypot(self.r, self.i)
    }

    fn abs_squared(&self) -> f32 {
        self.r * self.r + self.i * self.i
    }

    fn julia_iteration(&mut self, c: ComplexNumber) {
        *self = *self * *self + c;
    }

    fn compute_iterations(mut self, c: ComplexNumber) -> f32 {
        for i in 0..MAX_ITERATIONS {
            if self.abs_squared() > CUT_OFF_BOUND_SQUARED {
                return i as f32;
            }
            self.julia_iteration(c);
        }
        return MAX_ITERATIONS as f32;
    }
}


fn smooth_index(num_iterations: f32, c: &ComplexNumber) -> f32 {
    num_iterations + 1. - (f32::ln(2.) / c.abs() as f32) / f32::ln(2.)
}


fn julia_color(continuous_index: f32) -> Color {
    if continuous_index.round() as i32 == MAX_ITERATIONS {return BLACK;}

    let grad = colorgrad::CustomGradient::new().colors(
        &[
            colorgrad::Color::from_rgba8(102, 143, 204, 255),
            colorgrad::Color::from_rgba8(185, 83, 67, 255),
            colorgrad::Color::from_rgba8(255, 211, 47, 255),
            colorgrad::Color::from_rgba8(255, 255, 255, 255),
        ]).build().unwrap();
    let rgba = grad.at(continuous_index as f64 / MAX_ITERATIONS as f64).to_rgba8();

    return Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3]);
}


fn julia_black_white(num_iterations: f32) -> Color {
    let intensity = num_iterations / MAX_ITERATIONS as f32;

    Color {
        r: intensity,
        g: intensity,
        b: intensity,
        a: 1.,
    }
}

fn draw_julia(x_shift: f32, y_shift: f32, zoom: f32, c: ComplexNumber, colored: bool) {

    clear_background(BLACK);
    let (tx, rx) = mpsc::channel();

    for y in 0..(screen_height().ceil() as i32) {
        let tx = tx.clone();
        thread::spawn(move || {
            let val = draw_julia_row(x_shift, y_shift, zoom, c, y, colored);
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

fn draw_julia_row(
    x_shift: f32,
    y_shift: f32,
    zoom: f32,
    c: ComplexNumber,
    y: i32,
    colored: bool,
) -> (i32, Vec<Color>) {
    let mut colors = Vec::new();
    for x in 0..(screen_width().ceil() as i32) {
        // Type coercion 
        let (x, y) = (x as f32, y as f32);

        let point = ComplexNumber::new((x- x_shift) / zoom, -(y - y_shift)/ zoom);
        let num_iterations = point.compute_iterations(c);
        if colored {
            colors.push(julia_color(num_iterations));
        } else {
            colors.push(julia_black_white(num_iterations));
        }
    }
    return (y, colors);
    //     let (x, y) = (x as f32, y as f32);

    //     let mut iterations = Vec::new();
    //     let point = ComplexNumber::new(x - x_shift, -y - y_shift) / zoom;
    //     iterations.push(point.compute_iterations(c));
        
    //     println!("{:?}", iterations);
        
    //     for i in 0..ANTI_ALIASING_POINTS {
    //         let theta = (i as f32) / (ANTI_ALIASING_POINTS as f32) * std::f32::consts::TAU;
    //         let temp = ComplexNumber::new(f32::cos(theta), f32::sin(theta));
    //         let point = ComplexNumber::new(x as f32 - x_shift, -y as f32 - y_shift) + temp;

    //         iterations.push((point/zoom).compute_iterations(c) as f32);
    //     }
    //     let num_iterations = iterations.iter().sum::<f32>() / iterations.len() as f32;
    //     if colored {
    //         colors.push(julia_color(smooth_index(num_iterations, &c)));
    //     } else {
    //         colors.push(julia_black_white(num_iterations));
    //     }
    // }
    // return (y, colors);
}

fn change_zoom(zoom: &mut f32) -> bool {
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

fn change_center(x_shift: &mut f32, y_shift: &mut f32) -> bool {
    let mut moved = false;
    let movements = [
        (KeyCode::Left, 1., 0.),
        (KeyCode::Right, -1., 0.),
        (KeyCode::Up, 0., 1.),
        (KeyCode::Down, 0., -1.),
    ];

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

fn toggle_color(color_toggle: &mut bool) {
    if is_key_pressed(KeyCode::C) {
        *color_toggle = !*color_toggle;
    }
}

#[macroquad::main("Julia Sets :)")]
async fn main() {
    let mut zoom = 250.;
    let mut colored = false;
    let mut c = ComplexNumber::new(-0.391, -0.587);
    let mut x_shift = screen_width().ceil() / 2.;
    let mut y_shift = screen_height().ceil() / 2.;

    loop {
        println!("{}", get_frame_time());

        change_zoom(&mut zoom);
        change_center(&mut x_shift, &mut y_shift);
        change_c(&mut c);
        toggle_color(&mut colored);

        draw_julia(
            x_shift.clone(),
            y_shift.clone(),
            zoom.clone(),
            c.clone(),
            colored.clone(),
        );
        next_frame().await
    }
}
