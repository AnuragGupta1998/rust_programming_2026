use std::f32::consts::PI;
enum Shape{
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32),
}

//implimenting Enum 
impl Shape {

    fn area(&self)->f32{
        match self{
            Shape::Circle(radius) => 3.14*radius*radius,

            Shape::Rectangle(height,width ) => height*width,

            Shape::Square(side)=>side*side,

        }
    } 
}

fn area_of_given_shape(s: Shape) -> f32 {

    match s {
        Shape::Rectangle(height,width) => height * width,
        Shape:: Square(side) => side*side,
        Shape::Circle(radius) => PI*radius*radius, 
    }
}

fn main() {

    let area_of_circle = area_of_given_shape(Shape::Circle(5.0));
    println!("Area of Circle: {}", area_of_circle);

    let area_of_square = area_of_given_shape(Shape::Square(20.0));
    println!("Area of Square: {}", area_of_square);

    let are_of_rectangle = area_of_given_shape(Shape::Rectangle(10.0,5.0));
    println!("Area of Rectangle: {}", are_of_rectangle);

    //impl part of enum
    let r = Shape::Rectangle(10.0, 5.0);
    let s = Shape::Square(20.0);
    let c = Shape::Circle(5.0);

    println!("Circle={}", c.area());
    println!("Square={}",s.area());
    println!("Rectangle={}",r.area());


}