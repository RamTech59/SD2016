struct Point {
	x: f64,
	y: f64,
}
struct Solid {
	center: Point,
	radius: f64,
	speed: f64,

}
impl Point
{
	fn new(x:f64, y:f64) -> Point
	{
		Point{x: x, y: y}
	}
}
impl Solid
{
	fn new(c:Point, r:f64, s:f64) -> Solid
	{
		Solid{center: c, radius: r, speed: s}
	}
}
fn main(){
	let c = Point::new(0.0, 0.0);
	let r:f64 = 1.0;
	let s:f64 = 0.001;
	let s = Solid::new(c, r, s);
}
