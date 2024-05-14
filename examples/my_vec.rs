use macros::my_vec;

fn main() {
    let my_vec = my_vec! {1,2,3};
    println!("my_vec: {:?}", my_vec);

    let my_vec1 = my_vec!("a", "b", "C");
    println!("my_vec1: {:?}", my_vec1);

    let empty_vec: Vec<i32> = my_vec!();
    println!("empty_vec: {:?}", empty_vec);
}
