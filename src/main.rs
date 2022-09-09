use std::io::Read;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};


// read api key from env or config file in ~/.config/ipinfo
#[derive(Deserialize, Serialize)]
struct IpInfo {
    ip: String,
    hostname: String,
    anycast: bool,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
    timezone: String,
}




fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = App::new("ipinfo")
        .version("0.1.0")
        .author("Luke Milby")
        .about("Cli tool for ipinfo.io")
        .arg(Arg::with_name("ip")
            .long("ip")
            .takes_value(true)
            .help("IP address to lookup"))
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .takes_value(true)
            .env("IPINFO_KEY")
            .help("API Key for ipinfo.io")
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let key = matches.value_of("key").expect("missing key");

    let mut res = reqwest::blocking::get(format!("https://ipinfo.io/{}?token={}", ip, key))?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let payload: IpInfo = serde_json::from_str(body.as_str())?;
    let s = serde_json::to_string(&payload)?;

    println!("{}", s);
    Ok(())
}