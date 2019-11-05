use std::fs;

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


pub fn loader(filename: String, points: &mut Vec<Point>){
    let contents = fs::read_to_string(filename).expect("Something went wrong with file read");
    for line in contents.lines() {
        let mut s = line.split(',');
        let temp_x: i32 = s.next().unwrap().parse().unwrap();
        let temp_y: i32 = s.next().unwrap().parse().unwrap();
        points.push(Point{x:temp_x,y:temp_y});
    }
   
}

pub fn find_rec(points: Vec<Point>) -> u32 {
  let mut count: u32 = 0;
  if points.len() == 0 {return count};
  
  let c_points = points.to_vec();

  for p in points {
        let iter = c_points.iter().filter(|n| n.x > p.x && n.y > p.y);
        for q in iter {
            if c_points.contains(&Point{x:p.x,y:q.y}) &&
                c_points.contains(&Point{x:q.x,y:p.y}){
                count += 1;
            }
        }
  }
  return count;
}

