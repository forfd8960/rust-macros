use macros::EnumFrom;

#[allow(dead_code)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    value: T,
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(3).into();
    println!("{:?}", up);
}

impl<T> DirectionUp<T> {
    fn new(v: T) -> Self {
        Self { value: v }
    }
}
