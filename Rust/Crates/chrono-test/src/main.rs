use chrono::Utc;
use chrono::Local;
fn main() {

    println!("CHRONO UTC NOW: {}", Utc::now());
    println!("CHRONO LOCAL NOW : {}", Local::now());

    println!("NAIVE LOCAL : {}", Local::now().naive_local());
}
