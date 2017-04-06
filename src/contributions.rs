use std::process::Command;
use std::str;
use date;

//get username from gitconfig
fn get_username() -> Option<String> {
    let git_cmd = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--list")
        .output()
        .unwrap();
    let git_str = str::from_utf8(&git_cmd.stdout).unwrap();
    let git_vec: Vec<&str> = git_str.split('\n').collect();
    let git_username: Vec<&str>;

    //find user.name
    for i in 0..git_vec.len() {
        if git_vec[i].contains("user.name") {
            git_username = git_vec[i].split('=').collect();
            let username = git_username[1].to_string();
            return Some(username);
        }
    }
    None
}

pub fn get_today_contributions() -> Option<String> {
    //get date
    let today_string = date::get_date();
    let today = today_string.as_str();
    println!("Today: {}", today);

    //set username
    let username_string = get_username().unwrap();
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
