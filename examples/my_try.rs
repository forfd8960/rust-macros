// use anyhow::anyhow;
// use anyhow::Ok;
// use anyhow::Result;

fn main() {}

// fn main() -> Result<()> {
//     // let ret = f2(f1("xxx".to_string())?)?;
//     // println!("ret: {}", ret);

//     let ret1 = my_try!(f2(my_try!(f1("xxx".to_string()))));
//     println!("ret: {}", ret1);

//     Ok(())
// }

// fn f1(v: String) -> Result<String> {
//     Ok(format!("f1: {}", v))
// }

// fn f2(v: String) -> Result<String> {
//     Err(anyhow!("f2: {}", v))
// }

// // ? operator
// #[macro_export]
// macro_rules! my_try {
//     ($expr:expr) => {
//         match $expr {
//             std::result::Result::Ok(val) => val,
//             std::result::Result::Err(err) => return Err(err.into()),
//         }
//     };
// }
