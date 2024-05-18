use macros::AutoDebug;

#[derive(AutoDebug)]
pub struct Weather {
    city: String,
    temperature: i32,
    humidity: String,
    #[debug(skip)]
    wind_force: i32,
}

#[derive(AutoDebug)]
pub struct Time {
    hour: u8,
    minute: u8,
    seconds: u8,
}

fn main() {
    let weather = Weather {
        city: "Beijing".to_string(),
        temperature: 30,
        humidity: "dry".to_string(),
        wind_force: 1,
    };
    println!("{:?}", weather);
    println!("{:?}", weather.wind_force);

    let tm = Time {
        hour: 12,
        minute: 12,
        seconds: 12,
    };
    println!("{:?}", tm);
}
