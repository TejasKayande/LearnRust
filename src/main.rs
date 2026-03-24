
#[derive(Debug)] // This will help us print this struct
struct Dimentions {
    width: usize,
    height: usize,
}

fn get_area(dim: &Dimentions) -> usize {
    dim.width * dim.height 
}

fn main() -> () {

    let square = Dimentions {
        width: 25,
        height: 25
    };

    println!("Square {:?}" , square);
    println!("Square {:#?}", square);

    println!("Area of the square {} x {}: {}", square.width, square.height, get_area(&square));
}