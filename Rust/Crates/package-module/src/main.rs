mod shapes;

use shapes::rectangle::Rectangle;

fn main() {
    let rect1 : Rectangle = Rectangle::new(50, 50);
    let rect2 : Rectangle = Rectangle::new(100, 100);
    let rect3 : Rectangle = Rectangle::new(500, 500);

    rect1.print();
    rect2.print();
    rect3.print();

    println!("rect1 area = {} pixel(s)", rect1.area());
    println!("rect2 area = {} pixel(s)", rect2.area());
    println!("rect3 area = {} pixel(s)", rect3.area());

    println!("rect1 can hold rect3 ? {}", rect1.can_hold(&rect3));
    println!("rect3 can hold rect1 ? {}", rect3.can_hold(&rect1));

    //rect1.forbidden_print();
    // Rectangle::false_print();
    rect1.true_print();
}
