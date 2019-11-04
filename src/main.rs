mod pause;
mod loader;  
use loader::{loader, find_rec};

use pause::pause;


fn main() {
    
    let mut points: Vec<loader::Point> = Vec::new();

    println!("Ready to go?");
    pause();
    
    loader(&mut points);
   
    let num_rec = find_rec(points);
    // Show points
    
    println!("number of rectangles is {}",num_rec);    
}
    
