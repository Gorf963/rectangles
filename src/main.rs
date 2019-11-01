mod pause;
mod loader;  

use pause::pause;
use loader::{loader, find_rec};


fn main() {
    
    println!("Ready to go?");
    pause();
    
    let points = loader();

    // Show points
    for p in points {
        println!("x:{}, y:{}",p.x,p.y);    
    }

    //let count: u32 = find_rec(points);
    //println!("Count is {}",count);

}
