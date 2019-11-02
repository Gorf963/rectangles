
pub struct Point {
    pub x: i32,
    pub y: i32,
}


pub fn loader<'a>(points: &'a Vec<Point>) -> &'a Vec<Point> {
    
    points.push(new_point(1,1));
    points.push(new_point(2,1));
    points.push(new_point(3,1));
    points.push(new_point(1,2));
    points.push(new_point(2,2));
    points.push(new_point(3,2));
    return points;
}
//#[derive(Copy, Clone)]
pub fn new_point(x:i32, y:i32) -> Point{
    return Point {x,y,};
}
pub fn find_rec(points: Vec<Point>) -> u32 {
  let count: u32 = 0;
  return count;
}
