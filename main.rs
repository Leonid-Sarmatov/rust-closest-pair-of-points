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
        let mut point = Point::new(0, 0);
        point.x = read_integer();
        point.y = read_integer();
        vec_points.push(point);
    }

    sort_vec_of_the_points_for_x(&mut vec_points);

    println!("{:?}", vec_points);
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

fn printing_the_plane_whith_points(vec_points: Vec<Point>) {
    for i in 0..vec_points.len() {
        
    }
}

fn sort_vec_of_the_points_for_x(vec_points: &mut Vec<Point>) {
    vec_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
}

fn sort_vec_of_the_points_for_y(vec_points: &mut Vec<Point>) {
    vec_points.sort_unstable_by(|a, b| a.y.cmp(&b.y));
}

// Struct for points
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

// Metods for points
impl Point {
    // Constructor
    fn new(_x: i32, _y: i32) -> Point {
        Point { x: (_x), y: (_y) }
    }

    // Distance between two points
    fn calculate_distance(&self, point: Point) -> f64 {
        (((point.x-self.x)*(point.x-self.x) + 
        (point.y-self.y)*(point.y-self.y)) as f64).sqrt()
    }
}