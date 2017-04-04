use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process::Command;
use std::str;
use date;

fn get_username() -> String {
    //set file path
    let file_path = Path::new("user.txt");

    //open
    let mut file = File::open(&file_path).unwrap();

    //user.txt -> username
    let mut username = String::new();
    let _ = file.read_to_string(&mut username).unwrap();

    username
}

pub fn get_today_contributions() -> Option<String> {
    //get date
    let today_string = date::get_date();
    let today = today_string.as_str();
    println!("Today: {}", today);

    //set username
    let username_string = get_username();
    let username_str = username_string.as_str();
    let username = str::trim_matches(username_str, '\n');
    println!("User: {}", username);

    //get_contibution
    let url = ["https://github.com/users/", username, "/contributions"].join("");
    let curl_cmd = Command::new("curl").arg(url).output().unwrap();
    let curl_cmd_str = str::from_utf8(&curl_cmd.stdout).unwrap();

    //str -> str(array)
    let curl_cmd_split: Vec<&str> = curl_cmd_str.split('\n').collect();

    //find_today_contribution
    for i in 0..curl_cmd_split.len() {
        if curl_cmd_split[i].contains(&today) {
            let today_contribution = curl_cmd_split[i].to_string();
            return Some(today_contribution);
        }
    }
    None
}
