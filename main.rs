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
        println!("The point was saved (точка успешно записана)");
    }

    printing_the_plane_whith_points(&mut vec_points);
    sort_vec_of_the_points_for_x(&mut vec_points);
    println!("{:?}", recursive_seach_for_nearby_points(vec_points));
}

fn recursive_seach_for_nearby_points(vec_points: Vec<Point>) -> Vec<Point> {
    // Stopping recursion for length 3
    if vec_points.len() == 3 {
        let s1: f64 = vec_points[0].calculate_distance(vec_points[1].clone());
        let s2: f64 = vec_points[1].calculate_distance(vec_points[2].clone());
        let s3: f64 = vec_points[0].calculate_distance(vec_points[2].clone());
        let x: f64 = s1.min(s2.min(s3));
       
            if x == s1 {
                let mut res: Vec<Point> = Vec::new();
                res.push(Point::new(vec_points[0].x, vec_points[0].y));
                res.push(Point::new(vec_points[1].x, vec_points[1].y));
                return res;
            }
        
            if x == s2 {
                let mut res: Vec<Point> = Vec::new();
                res.push(Point::new(vec_points[1].x, vec_points[1].y));
                res.push(Point::new(vec_points[2].x, vec_points[2].y));
                return res;
            }
        
            if x == s3 {
                let mut res: Vec<Point> = Vec::new();
                res.push(Point::new(vec_points[0].x, vec_points[0].y));
                res.push(Point::new(vec_points[2].x, vec_points[2].y));
                return res;
            }
        
    }

    // Stopping recursion for length 2
    if vec_points.len() == 2 {
        let mut res: Vec<Point> = Vec::new();
        res.push(Point::new(vec_points[0].x, vec_points[0].y));
        res.push(Point::new(vec_points[1].x, vec_points[1].y));
        return res;
    }

    // Sending halves of the vectors further into recursion
    let a: Vec<Point> = recursive_seach_for_nearby_points(
        (&vec_points[..(vec_points.len()/2)]).to_vec()
    );
    let b: Vec<Point> = recursive_seach_for_nearby_points(
        (&vec_points[(vec_points.len()/2)..]).to_vec()
    );

    //println!("{}", 79);
    let x: f64 = a[0].calculate_distance(a[1].clone());
    //println!("{}", 81);
    let y: f64 = b[0].calculate_distance(b[1].clone());

    // Check the case when the nearest points are in different halves of the vector
    let c: Vec<Point> = confluence(
        &vec_points,
        (vec_points[vec_points.len()/2-1].x + 
        vec_points[vec_points.len()/2].x) as f64/2.0,
        x.min(y)
    );

    let z: f64 = c[0].calculate_distance(c[1].clone());

    let _min: f64 = (x.min(y)).min(z);
    if _min == x { return a;}
    if _min == y { return b;}
    return c;
    
}

fn confluence(vec_points: &Vec<Point>, mu: f64, s: f64) -> Vec<Point> {
    // Create the resulting points between which the distance will be the shortest
    let mut r1: Point = Point::new(i32::MAX, i32::MIN);
    let mut r2: Point = Point::new(i32::MIN, i32::MAX);

    // Create vector for points falling into the scatter
    let mut zip_vector: Vec<Point> = vec_points
        .iter()
        .filter(|i| i.x as f64 >= mu-0.5*s && i.x as f64 <= mu+0.5*s)
        .cloned()
        .collect();

    // Sort this vector along the Y axis
    sort_vec_of_the_points_for_y(&mut zip_vector);

    if zip_vector.len() != 0 {
        let mut _min: f64 = s;
        for i in 0..zip_vector.len().wrapping_sub(1) {
            //println!("{}", 123);
            if _min > zip_vector[i].calculate_distance(zip_vector[i+1].clone()) {
                r1.x = zip_vector[i].x;
                r1.y = zip_vector[i].y;

                r2.x = zip_vector[i+1].x;
                r2.y = zip_vector[i+1].y;

                _min = zip_vector[i].calculate_distance(zip_vector[i+1].clone());
            }
        }
    }
    let mut result: Vec<Point> = Vec::new();
    result.push(r1);
    result.push(r2);

    return result;
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

fn printing_the_plane_whith_points(vec_points: &mut Vec<Point>) {
    sort_vec_of_the_points_for_x(vec_points);
    let min_x: i32 = vec_points[0].x;
    let max_x: i32 = vec_points[vec_points.len()-1].x;

    sort_vec_of_the_points_for_y(vec_points);
    let min_y: i32 = vec_points[0].y;
    let max_y: i32 = vec_points[vec_points.len()-1].y;

    let mut vec_matrix: Vec<Vec<String>> = Vec::new();

    for x in min_y..=max_y {
        let mut vec_arr: Vec<String> = Vec::new();
        for y in min_x..=max_x {
           vec_arr.push(String::from(" "))
        }
        vec_matrix.push(vec_arr);
    }
    
    for i in vec_points {
        vec_matrix[(i.y-min_y) as usize][(i.x-min_x) as usize] = String::from("*");
    }

    print!("  ");
    for i in min_x..=max_x {
        print!("{} ", i)
    }

    println!();
    for i in min_y..=max_y {
        print!("{} ", i);
        for j in 0..vec_matrix[(i-min_y) as usize].len() {
            print!("{} ", vec_matrix[(i-min_y) as usize][j]);
        }
        println!();
    }
}

fn sort_vec_of_the_points_for_x(vec_points: &mut Vec<Point>) {
    vec_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
}

fn sort_vec_of_the_points_for_y(vec_points: &mut Vec<Point>) {
    vec_points.sort_unstable_by(|a, b| a.y.cmp(&b.y));
}

// Struct for points
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

// Metods for points
impl Point {
    // Constructor
    fn new(_x: i32, _y: i32) -> Self {
        Point { x: (_x), y: (_y) }
    }

    // Distance between two points
    fn calculate_distance(&self, point: Point) -> f64 {
        //println!("{}, {}, {}, {}\n", self.x, self.y, point.x, point.y);
        //(((point.x-self.x)*(point.x-self.x) + 
        //(point.y-self.y)*(point.y-self.y)) as f64).sqrt()

        let dx = point.x.wrapping_sub(self.x);
        let dy = point.y.wrapping_sub(self.y);

        ((dx.wrapping_mul(dx) + dy.wrapping_mul(dy)) as f64).sqrt()
    }

    // Equals of two points
    fn equals(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}