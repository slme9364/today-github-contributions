use std::process::Command;
use std::str;
use date;

pub fn get_today_contributions() -> Option<String> {
    //get date
    let today_string = date::get_date();
    let today = today_string.as_str();
    println!("Today: {}", today);

    //get_contibution
    let url = "https://github.com/users/slme9364/contributions";
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
