mod pause;
mod loader;  

use loader::{loader, find_rec};
use pause::pause;
use std::env;
use std::time::Instant;

fn main() {
    
    let mut points: Vec<loader::Point> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    println!("Ready to go?");
    pause();
    let start = Instant::now();
    loader(filename.clone(), &mut points);
    println!("load took: {:?}",start.elapsed());
    let start = Instant::now();

    let num_rec = find_rec(points);
    let duration = start.elapsed();
    println!("number of rectangles is {}",num_rec);    
    println!("process took: {:?}",duration);
}
    
