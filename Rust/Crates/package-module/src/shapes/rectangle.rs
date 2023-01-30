#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }

    fn forbidden_print(&self) {
        println!("forbidden print");
    }

    fn false_print() {
        println!("false print");
    }

    pub fn true_print(&self) {
        self.forbidden_print();
        Rectangle::false_print();
    }
}