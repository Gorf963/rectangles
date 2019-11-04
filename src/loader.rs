#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


pub fn loader(points: &mut Vec<Point>){
    
    points.push(Point{x:1,y:1});
    points.push(Point{x:2,y:1});
    points.push(Point{x:3,y:1});
    points.push(Point{x:1,y:2});
    points.push(Point{x:2,y:2});
    points.push(Point{x:3,y:2});
    
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

