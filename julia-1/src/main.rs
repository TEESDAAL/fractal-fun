use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
struct ComplexNumber {
    r: f64,
    i: f64,
}

const MAX_ITERATIONS: i32 = 10000;
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

#[macroquad::main("BasicShapes")]
async fn main() {
    let c = ComplexNumber::new(-0.391, -0.587);
    // println!("{:?}", c.compute_iterations(c.clone()));
    
    loop {
        clear_background(BLACK);
        let shift = screen_height().ceil() as i32 / 2;
        for y in 0..(screen_height().ceil() as i32) {
            for x in 0..(screen_width().ceil() as i32) {
                let point = ComplexNumber::new((x - shift) as f64 / 1000., -(y - shift) as f64/ 1000.);
                let num_iter = point.compute_iterations(c);
                let pixel_color = julia_color(num_iter);
                // if num_iter != 0 {println!("{:?}", pixel_color);};

                draw_rectangle(x as f32, y as f32, 1., 1., pixel_color)
            }
        }
        next_frame().await
    }
}
