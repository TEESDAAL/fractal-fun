use macroquad::prelude::*;
use std::{thread, time};

struct Triangle {
    x: f32,
    y: f32,
    height: f32,
}
impl Triangle {
    fn new(x: f32, y: f32, height: f32) -> Self {
        Self {
            x,
            y,
            height,
        }
    }

    fn width(&self) -> f32 {
        1./f32::sqrt(3.) * self.height
    }
    
    fn verticies(&self) -> (Vec2,Vec2, Vec2){
        (
            Vec2::new(self.x - self.width(), self.y + self.height/2.),
            Vec2::new(self.x, self.y - self.height/2.),
            Vec2::new(self.x + self.width(), self.y + self.height/2.),
        )
    }
    
    fn draw(&self, color: Color) {
        let (v1, v2, v3) = self.verticies();
        draw_triangle(v1, v2, v3, color)
    }
}

fn draw_serpinski(triangle: &Triangle, depth: u32) {
    if depth == 0 {
        return;
    }
    let cutout_triangle = Triangle::new(
        triangle.x,
        triangle.y + triangle.height/4.,
        -triangle.height /2.,
    );
    cutout_triangle.draw(BLACK);

    let sub_triangle = Triangle::new(triangle.x, triangle.y - triangle.height/4., triangle.height/2.);
    draw_serpinski(&sub_triangle, depth-1);
    let sub_triangle = Triangle::new(triangle.x + triangle.width()/2., triangle.y + triangle.height/4., triangle.height/2.);
    draw_serpinski(&sub_triangle, depth-1);
    let sub_triangle = Triangle::new(triangle.x - triangle.width()/2., triangle.y + triangle.height/4., triangle.height/2.);
    draw_serpinski(&sub_triangle, depth-1);

}
#[macroquad::main("Poly-Gaskets")]  
async fn main() {
    let start_triangle = Triangle::new(screen_width() / 2.,screen_height() / 2., 600.);
    loop {
        start_triangle.draw(WHITE);
        draw_serpinski(&start_triangle, 8);
        next_frame().await
    }
}
