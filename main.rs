use std::io;

/*
A program for finding points on a plane with 
the smallest distance between them.
 */
fn main() {

    println!("Enter the number of points (введите количество точек)");

    // Read quantity of points
    let quantity_of_points: i32 = read_integer();

    // Create vector for array of points
    let mut vec_points = Vec::new();

    // Filling the vector if points
    println!("Enter your points (введите точки)");
    for i in 0..quantity_of_points {

        // Array of coordinates
        let mut arr: [i32; 2] = [0, 0];
        arr[0] = read_integer();
        arr[1] = read_integer();
        vec_points.push(arr)
    }

    println!("{:?}", vec_points)
}

fn read_integer() -> i32 {
    loop {
        // Create string variable for read input
        let mut input_string = String::new();

        // Writing input line to variable
        io::stdin().read_line(&mut input_string).expect("Error: Can not read line!");

        // Parse string to integer
        match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Error: Can not parse input line to i32!"),
        }
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(_x: i32, _y: i32) -> Point {
        Point { x: (_x), y: (_y) }
    }

    fn calculate_distance(&self, point: Point) -> f64 {
        (((point.x-self.x)*(point.x-self.x) + 
        (point.y-self.y)*(point.y-self.y)) as f64).sqrt()
    }
}