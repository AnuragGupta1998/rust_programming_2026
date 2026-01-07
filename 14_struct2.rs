//definition of a struct or structure
struct Rectangle{
    height:f32,
    width:f32,
}

// implementation block for the Rectangle struct
impl Rectangle{

    //these are the member functions or methods that given below

    // method to calculate area 
    //only this method can acess the struct's fields because of &self keyword
    fn area(&self) -> f32{
        self.height * self.width
    }

    fn perimeter_of_rectangle(&self) -> f32{
        2.0 * (self.height + self.width) //we can also return with return keyword and semocolon
    }

    //static method
    fn print_static(){
        println!("This is a static method");    
    }

    //method that takes parameters
    fn print_demo(a:u32){
        println!("The value of a is: {}", a);
    }

    //method that returns a value
    fn demo_return_value(a:u32) -> u32{
        a + 10
    }   
}

fn main(){

    let r1 =Rectangle{
        height:10.0,
        width:5.0,
    };

    let area = r1.area();
    println!("The area of the rectangle is: {}", area);

    let primeter = r1.perimeter_of_rectangle();
    println!("The perimeter of the rectangle is: {}", primeter);

    //we can not access other member functions directly
    // Rectangle::area(); // this will give error
    Rectangle::print_static();
    Rectangle::print_demo(25);  
    let result = Rectangle::demo_return_value(15);
    println!("The result from demo_return_value is: {}", result);

}