fn main() {}
// fn main() {
//     let my_vec = my_vec! {1,2,3};
//     println!("my_vec: {:?}", my_vec);

//     let my_vec1 = my_vec!("a", "b", "C");
//     println!("my_vec1: {:?}", my_vec1);

//     let empty_vec: Vec<i32> = my_vec!();
//     println!("empty_vec: {:?}", empty_vec);

//     let my_i32_vec = my_vec![10; 0];
//     println!("my_i32_vec: {:?}", my_i32_vec);

//     let my_comma_vec = my_vec!["a", "b", "c", "d"];
//     println!("my_comma_vec: {:?}", my_comma_vec);
// }

// #[macro_export]
// macro_rules! my_vec {
//     () => {Vec::new()};
//     ($elem:expr; $n:expr) => { std::vec::from_elem($elem,$n) };
//     ($($x:expr),+ $(,)?) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(temp_vec.push($x);)*
//             temp_vec
//         }
//     };
// }
