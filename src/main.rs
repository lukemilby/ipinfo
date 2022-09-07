use std::env;
use std::io::Read;
use clap::{App, Arg};
// use serde_derive::Deserialize;
// use serde_derive::Serialize;

// read api key from env or config file in ~/.config/ipinfo

// #[derive(Default, Debug, Clone, PartalEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct IpInfo {
//     ip: String,
//     hostname: String,
//     anycast: bool,
//     city: String,
//     region: String,
//     country: String,
//     loc: String,
//     org: String,
//     postal: String,
//     timezone: String,
// }


// error_chain! { // TODO: What is this?
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = App::new("ipinfo")
        .version("0.1.0")
        .author("Luke Milby")
        .about("Cli tool for ipinfo.io")
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .takes_value(true)
            .help("API Key for ipinfo.io"))
        .arg(Arg::with_name("ip")
            .long("ip")
            .takes_value(true)
            .help("IP address to lookup"))
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let api_key = matches.value_of("key").unwrap();

    let mut res = reqwest::blocking::get(format!("https://ipinfo.io/{}?token={}", ip, api_key))?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
