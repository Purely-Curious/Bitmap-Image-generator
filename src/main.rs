#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
// Image library.
// Use the formula for computing the mandelbrot set.
// Set the pixel position of converging points to white and other points to black.
mod bmp_definitions;
use bmp_definitions::*;
//  Z(n+1) = [Z(n)^2 + c]
// Z is a complex number: (x, y)
// let z = x + y * i
// z^ 2 = (x + y*i)^2
// x^2 + 2 * x * y * i + (y * i) ^ 2
// z^2 = (x^2 - y^2) + (2 * x * y) * i

// x_n+1 = (x_n^2 - y_n^2)
// y_n+1 = (2 * x_n * y_n) * i

// Number of iterations: 1000 as a starting point.

// set image width, image height;
// create an array of u8 with size [image.width * image.height]
// begin iteration:
// for i in 0..image.height
//      for j in 0..image.width
//             perform calculation.
// For png
//let buffer = [137, 80, 78, 71, 13, 10, 26, 10];

use std::ops::{Add, Div, Mul, Sub};
// u8 u16 u32
// u8 -> 1 byte
// u16 -> 2 bytes
// u32 -> 4 bytes
// 0000 0000
// 0000 56D0
// 0000 0001
// 0001 0000
// TODO: fuck it, i'm using a 32 bit number for colors.
// fn intensity(array: &mut Vec<u32>.
#[derive(Clone, Copy, Debug)]
struct Point
{
    x: f32,
    y: f32,
}
impl Point
{
    fn new(x: f32, y: f32) -> Self
    {
        Self { x, y, }
    }
    fn norm(&self) -> f32
    {
        let px = self.x * self.x;
        let py = self.y * self.y;
        (px + py).sqrt()
    }
    
    fn dragon_iter(&self, c: Point, angle: f32) -> Point
    {
        Point
        {
            x: (angle.cos() * self.x - angle.sin() * self.y + c.x),
            y: (angle.cos() * self.x + angle.sin() * self.y + c.y)
        }
    }
}
impl Sub for Point
{
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Point
{
    // x_n+1 = (x_n^2 - y_n^2)
    // y_n+1 = (2 * x_n * y_n) * i
    type Output = Point;
    fn mul(self, rhs: Self) -> Self::Output {
        let new_x = self.x.abs() * rhs.x.abs() - self.y.abs() * rhs.y.abs();
        let new_y = self.x.abs() * rhs.y.abs() + self.y.abs() * rhs.x.abs();
        Self::new(new_x,
                 new_y)
    }
}
impl Add for Point
{
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output
    {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Div for Point
{
    type Output = Point;
    fn div(self, deno: Self) -> Self::Output
    {
        let new_x = (self.x * deno.x + self.y + deno.y) / (deno.x * deno.x - deno.y * deno.y);
        let new_y = (self.y * deno.x - deno.y * self.x) / (deno.x * deno.x - deno.y * deno.y);
        Self::new(new_x, new_y)
    }
}
fn fractal(image: &mut BmpImage, width: u32, height: u32)
{
    let scalex = 3.0 / width as f32;
    let scaley = 3.0 / height as f32;
    let iteration_limit = 255 * 6; //10_000_000;//255;
    for y in 0..height
    {
        for x in 0..width
        {
            //let color = 0xfe0d00;
           // image.set_pixel_data(x, y, color)
        }
    }
    for y in 0..height
    {
        for x in 0..width
        {
            let cx = x as f32 * scalex - 1.5;
            let cy = y as f32 * scaley - 1.5;
            // For julia set
            //let c: Point = Point::new(-0.75, 0.11);//Point::new(-0.4, 0.6);
            //let mut z: Point = Point::new(cx, cy);
            // For Mandelbrot set
            let c: Point = Point::new(cx, cy);
            let mut z: Point= Point::new(0.0, 0.0);
            let mut i = 0;
            while (i < iteration_limit) && z.norm() <= 2.0//1.616
            {
                //z = (z * z * c) + c;
                //z = (z * c) + (z / c);
                z = (z * c * c) + z;
                //let deno: Point = (z - (z * z) / Point::new(2.0, 0.0));

                //z = ((Point::new(1.0, 0.0) - (z * z * z) / Point::new(6.0, 0.0)) / (deno * deno)) + Point::new(2.0, 0.0);
                //z = z.dragon_iter(c, 135.0);//((x + y) % 360) as f32 * 1.0);
                //print!("{:?}", z);
                i +=1;
            }
            //println!("{}", i);
            //let mut color =  (((i as f32 / (iteration_limit as f32)) * 0xffffff as f32) as u32);//((i as u32) << 16 | ((i as u32 & 0xff)));
            let mut color = (i as u32) << 16 | (i as u32) << 24 | i as u32;
    
            if (i == iteration_limit)
            {
                color = 0x0;
            } 
            image.set_pixel_data(x, y, color);
        }
    }
}


/* 
fn iteration_for_set(array: &mut Vec<u32>) {
    let mut x: u8;
    let mut y: u8;

    for _ in 0..1000 {
        for col in 0..IMAGE_HEIGHT {
            for row in 0..IMAGE_WIDTH {
                let idx: usize = (col + 1) as usize * row as usize;
                x = (col * col - row * row);
                y = (2 * col * row);
                array[idx] = (x + y) as u32 % 255;
                //
            }
        }
    }
}
*/


fn main() {//-> std::io::Result<()> {
    let width: u32 = 512 * 2;
    let height: u32 = 480 * 2;
    let size = width * height;
    let color: u32 =  0xffffff;//0xffeedd;//0xffffff;//0xffeedd;//0xFFFFFF; //0xEEFFAD;
    let data: Vec<u32> = vec![color; size as usize];
    //intensity(&mut data, width as usize, height as usize, size as usize, color);
    println!("{}", data.len());
    let mut image = BmpImage::create_bitmap_image("fractal3.bmp", width, height, data);
    //intensity(&mut image, width, height, size as usize, color);
    fractal(&mut image, width, height);
    image.to_file();
    //Ok(())
}
