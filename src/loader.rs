
pub fn loader() -> &'static Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    points.push(new_point(1,1));
    points.push(new_point(2,1));
    points.push(new_point(3,1));
    points.push(new_point(1,2));
    points.push(new_point(2,2));
    points.push(new_point(3,2));
    &points
}
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub fn new_point(x:i32, y:i32) -> Point{
    return Point {x,y,};
}
pub fn find_rec(points: Vec<Point>) -> u32 {
  let count: u32 = 0;
  return count;
}
