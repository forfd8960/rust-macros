use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;

use macros::my_try;

fn main() -> Result<()> {
    // let ret = f2(f1("xxx".to_string())?)?;
    // println!("ret: {}", ret);

    let ret1 = my_try!(f2(my_try!(f1("xxx".to_string()))));
    println!("ret: {}", ret1);

    Ok(())
}

fn f1(v: String) -> Result<String> {
    Ok(format!("f1: {}", v))
}

fn f2(v: String) -> Result<String> {
    Err(anyhow!("f2: {}", v))
}
