use std::f64;
use std::f64::consts::PI;

struct Pos {
  x: f64,
  y: f64,
}

fn main() {
  let p = Pos { x: 1.0, y: 1.0 };
  let r: f64 = 90.0;

  let result = dispatch_rotate(p, PI / 2.0);
  println!("{},{}", result.x, result.y);
}

fn dispatch_rotate(pos: Pos, r: f64) -> Pos {
  println!("(sin{},cos{})=({},{})", &r, &r, &r.sin(), &r.cos());
  return Pos {
    x: (pos.x * r.cos() - pos.y * r.sin()),
    y: (pos.x * r.sin() + pos.y * r.cos()),
  };
}
