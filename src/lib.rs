#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr; $n:expr) => { std::vec::from_elem($elem,$n) };
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

// ? operator
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            std::result::Result::Ok(val) => val,
            std::result::Result::Err(err) => return Err(err.into()),
        }
    };
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
