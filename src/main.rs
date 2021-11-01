use reqwest::Client;
use serde::Serialize;
use std::env;
use std::process;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[derive(Serialize, Debug)]
struct ProfileStatus {
    status_text: String,
    status_emoji: String,
    // status_expiration: String
}

#[derive(Serialize, Debug)]
struct ProfileRequest {
    user: String,
    profile: ProfileStatus,
}

#[tokio::main]
async fn main() {
    let access_token = "AJIPSY_ACCESS_TOKEN";
    let member_id = "AJIPSY_MEMBERID";

    let token = match env::var(access_token) {
        Ok(val) => val,
        Err(_) => {
            println!("Cannot read access token.");
            process::exit(1);
        }
    };

    let target = match env::var(member_id) {
        Ok(val) => val,
        Err(_) => {
            println!("Connot read username.");
            process::exit(1);
        }
    };

    let r25_help = format!("Set status '{} Room 2525'", '\u{1f3eb}');
    let r27_help = format!("Set status '{} Room 2719'", '\u{1f3eb}');
    let home_help = format!("Set status '{} Home'", '\u{1f3e0}');
    let out_help = format!("Set status '{} Going out'", '\u{1f6b6}');

    let cmd = app_from_crate!()
        .subcommand(App::new("room2525").about(&*r25_help))
        .subcommand(App::new("room2719").about(&*r27_help))
        .subcommand(App::new("home").about(&*home_help))
        .subcommand(App::new("out").about(&*out_help))
        .subcommand(App::new("reset").about("Reset Status"))
        .arg(Arg::from_usage(
            "[TEXT] -t --text [TEXT] 'Custom status text'",
        ))
        .arg(Arg::from_usage(
            "[EMOJI] -e --emoji [EMOJI] 'Custom status emoticon, like `:school:`'",
        ))
        .setting(AppSettings::DeriveDisplayOrder);

    let matches = cmd.get_matches();

    if let Some(_matches) = matches.subcommand_matches("room2525") {
        let req = build_request(&target, "Room2525", ":school:");
        post_status(&token, &req).await;
    }

    if let Some(_matches) = matches.subcommand_matches("room2719") {
        let req = build_request(&target, "Room2719", ":school:");
        post_status(&token, &req).await;
    }

    if let Some(_matches) = matches.subcommand_matches("home") {
        let req = build_request(&target, "Home", ":house:");
        post_status(&token, &req).await;
    }

    if let Some(_matches) = matches.subcommand_matches("out") {
        let req = build_request(&target, "Going out", ":walking:");
        post_status(&token, &req).await;
    }

    if let Some(_matches) = matches.subcommand_matches("reset") {
        let req = build_request(&target, "", "");
        post_status(&token, &req).await;
    }

    let custom_text = matches.value_of("TEXT").unwrap_or("");

    let custom_emoji = matches.value_of("EMOJI").unwrap_or("");

    let req = build_request(&target, custom_text, custom_emoji);

    post_status(&token, &req).await
}

fn assert_emoji_string(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    if !s.starts_with(':') {
        assert_emoji_string(format!(":{}", s));
    }

    if !s.ends_with(':') {
        assert_emoji_string(format!("{}:", s));
    };

    s
}

fn build_request(user: &str, text: &str, emoji: &str) -> ProfileRequest {
    ProfileRequest {
        user: user.to_string(),
        profile: ProfileStatus {
            status_text: text.to_string(),
            status_emoji: assert_emoji_string(emoji.to_string()),
        },
    }
}

async fn post_status(token: &str, request: &ProfileRequest) {
    let endpoint = "https://slack.com/api/users.profile.set";
    let client = Client::new();
    let res = client
        .post(endpoint)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .json(request)
        .send()
        .await;

    match res {
        Ok(_) => {
            println!("Success!!");
            process::exit(0);
        }
        Err(e) => {
            println!("Failed... {:?}", e);
            process::exit(1);
        }
    };
}
