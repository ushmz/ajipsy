use std::env;
use std::process;
use reqwest::{Client, Error, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate clap;
use clap::{Arg, ArgGroup};

#[derive(Serialize, Deserialize, Debug)]
struct ProfileStatus {
    status_text: String,
    status_emoji: String,
    // status_expiration: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ProfileRequest {
    user: String,
    profile: ProfileStatus
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let access_token = "AJIPSY_ACCESS_TOKEN";
    let member_id = "AJIPSY_MEMBERID";

    let token = match env::var(access_token) {
        Ok(val) => val,
        Err(_) => {
            println!("Cannot read access token.");
            process::exit(1);
        },
    };

    let target = match env::var(member_id) {
        Ok(val) => val,
        Err(_) => {
            println!("Connot read username.");
            process::exit(1);
        },
    };
 
    let cmd = app_from_crate!()
        .arg(Arg::with_name("room2525")
             .help("Set status 'Room 2525'")
             .long("room2525"))
        .arg(Arg::with_name("room2719")
             .help("Set status 'Room 2719'")
             .long("room2719"))
        .arg(Arg::with_name("home")
             .help("Set status 'home'")
             .long("home"))
        .arg(Arg::with_name("out")
             .help("Set status 'out'")
             .long("out"))
       .group(ArgGroup::with_name("preset")
            .args(&["room2525", "room2719", "home", "out"]))
        .arg(Arg::from_usage("[TEXT] -t --text [TEXT] 'Status text'"))
        .arg(Arg::from_usage("[EMOJI] -e --emoji [EMOJI] 'Status emoticon(like :school:)'"));

    let matches = cmd.get_matches();

    let mut status_text: &str = "";
    let mut status_emoji: &str = "";

    if matches.is_present("preset") {
        let ( r25, r27, hm, out) = (matches.is_present("room2525"),
                                    matches.is_present("room2719"),
                                    matches.is_present("home"),
                                    matches.is_present("out"));
        println!("{}, {}, {}, {}", r25, r27, hm, out);
        status_text = if r25 {"Room2525"} else if r27 {"Room2719"} else if hm {"Home"} else if out {"Going out"} else {""};
        status_emoji = if r25 {":school:"} else if r27 {":school:"} else if hm {":house:"} else if out {":walking:"} else {""};
    }

    match matches.value_of("TEXT") {
        Some(val) => status_text = val,
        None => {}
    };

    match matches.value_of("EMOJI") {
        Some(val) => status_emoji = val,
        None => {}
    };

    let req = ProfileRequest {
        user: target.to_string(),
        profile: ProfileStatus {
            status_text: status_text.to_string(),
            status_emoji: assert_emoji_string(status_emoji.to_string())
        }
    };

    let res = post_status(&token, &req).await?;
    match res.status() {
        StatusCode::OK => {
            println!("Success!!");
            Ok(())
        }
        s => {
            println!("Error: Status code {:?}", s);
            process::exit(1)
        }
    }
}

fn assert_emoji_string(s: String) -> String {
    // Not Smart!!!!!
    if s.is_empty() {return s;}
    if s.starts_with(":") {
        if s.ends_with(":") {
            return s;
        } else {
            return format!("{}:", s);
        }
    } else {
         if s.ends_with(":") {
            return format!(":{}", s);
        } else {
            return format!(":{}:", s);
        }
    };
}

async fn post_status(token: &str, request: &ProfileRequest) -> Result<Response, Error> {
    let endpoint = "https://slack.com/api/users.profile.set";
    let client = Client::new();
    client.post(endpoint)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .json(request)
        .send()
        .await
}
