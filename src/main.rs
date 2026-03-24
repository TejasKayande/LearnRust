
struct Dimentions {
    width: usize,
    height: usize,
}

impl Dimentions {
    fn area(&self) -> usize {
        self.width * self.height
    }
}


// structs can have multiple impl blocks.
impl Dimentions {

    fn create(&self, w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

fn main() -> () {

    let square = Dimentions {
        width: 25,
        height: 25
    };

    println!("Area of the square {} x {}: {}", square.width, square.height, square.area());

    let mut square2: Dimentions = Dimentions { width: 0, height: 0 };
    square2 = square2.create(30, 30);

    println!("Area of the square {} x {}: {}", square2.width, square2.height, square2.area());
}