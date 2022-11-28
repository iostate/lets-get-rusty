#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // the area function is associated w/ Rectangle now
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 35,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 15,
    };

    // Calculating area and checking if rect can hold rect2 & rect3
    println!("rect: {:#?} area: {:#?}\n", rect, rect.area());
    println!("rect2: {:#?} area: {:#?}\n", rect2, rect2.area());
    println!("Can rect hold rect2? : {}\n", rect.can_hold(&rect2));
    println!("rect3: {:#?} area: {:#?}\n", rect3, rect3.area());
    println!("Can rect hold rect3? : {}\n", rect.can_hold(&rect3));

    // Notice how we use dot.notatiion to call method on an instnace
    // in C++ there's a different way to call a method on an Object directly
    // or to call a metjod on a pointer to an object.

    // In Rust, doing these two things are the same. Rust does this through automatic
    // referencing and dereferencing.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
