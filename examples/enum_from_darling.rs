use macros::EnumFromDarling;

#[allow(dead_code)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Right(T),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    value: T,
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(3).into();
    println!("up: {:?}", up);

    let right: Direction<i32> = 10.into();
    println!("right: {:?}", right);
}

impl<T> DirectionUp<T> {
    fn new(v: T) -> Self {
        Self { value: v }
    }
}
