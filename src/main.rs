use postgres::{Client, NoTls};
use reqwest::blocking::get;
use std::time::Duration;
use std::{env, thread};

fn main() {
    loop {
        let client = Client::connect(&*env::var("HEART_CONN").unwrap(), NoTls);
        match client.is_err() {
            false => {
                let resp = get(env::var("HEART_URL").unwrap());
                resp.expect("Heartbeat failed!");
                println!("Sent heartbeat")
            }
            true => {
                println!("Connection error occurred. Not sending Heartbeat");
                println!("Error: {:?}", client.as_ref().err())
            }
        }
        thread::sleep(Duration::from_secs(
            env::var("HEART_SEC").unwrap().parse::<u64>().unwrap(),
        ));
    }
}
