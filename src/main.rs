mod pause;
//mod loader;  
//use loader::{loader, find_rec};

use pause::pause;

struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    
    let mut points: Vec<Point> = Vec::new();

    println!("Ready to go?");
    pause();
    
//    loader(&points);
    points.push(Point {x:1,y:1});
    points.push(Point {x:2,y:1});
    points.push(Point {x:3,y:1});
    points.push(Point {x:1,y:2});
    points.push(Point {x:2,y:2});
    points.push(Point {x:3,y:2});

    
    // Show points
    for p in points {
        println!("x:{}, y:{}",p.x,p.y);    
        //points.contains(x: &T)
    }

    

}
    
