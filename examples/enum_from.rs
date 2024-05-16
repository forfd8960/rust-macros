use macros::EnumFrom;

#[allow(dead_code)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp {
    value: i32,
}

fn main() {
    let up: Direction = Direction::Up(DirectionUp::new(3));
    println!("{:?}", up);
}

impl DirectionUp {
    fn new(v: i32) -> Self {
        Self { value: v }
    }
}
