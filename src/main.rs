struct Pos {
    x: i32,
    y: i32,
}
fn main() {
    let p = Pos { x: 2, y: 1 };
    let r = rotate(p, 90);

    println!("{},{}", r.x, r.y);
}

fn rotate(pos: Pos, dir: i32) -> Pos {
    println!("{}", &dir);
    return Pos {
        x: pos.x * 2,
        y: pos.y * 2,
    };
}
