use macros::AutoDeref;

#[derive(Debug, AutoDeref)]
#[deref(mutable = true, field = "inner")]
pub struct RespBulkString {
    inner: String,
}

fn main() {
    let mut resp = RespBulkString {
        inner: "test".to_string(),
    };
    println!("{:?}", resp);

    *resp = "xyz".to_string();
    println!("resp: {:?}", resp);
}
