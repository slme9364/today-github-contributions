use std::process::Command;
use std::str;

fn num_month(month_str: &str) -> &str {
    match month_str {
        "Jan" => "1",
        "Feb" => "2",
        "Mar" => "3",
        "Apr" => "4",
        "May" => "5",
        "Jun" => "6",
        "Jul" => "7",
        "Aug" => "8",
        "Sep" => "9",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => "0",
    }
}

fn get_date() -> String {
    let date_cmd = Command::new("date").output().unwrap();

    let mut date_cmd_str = str::from_utf8(&date_cmd.stdout).unwrap();

    date_cmd_str = str::trim_matches(date_cmd_str, '\n');
    println!("Today: {}", date_cmd_str);

    let mut date_cmd_split: Vec<&str> = date_cmd_str.split(' ').collect();

    //github date -> yyyy-mm-dd
    //month Engish -> num
    date_cmd_split[1] = num_month(&date_cmd_split[1]);
    let date_string: String = [date_cmd_split[5], date_cmd_split[1], date_cmd_split[2]].join("-");

    date_string
}

fn main() {
    //get contributions
    let url = "https://github.com/users/slme9364/contributions";
    let curl_cmd = Command::new("curl").arg(url).output().unwrap();
    let curl_cmd_str = str::from_utf8(&curl_cmd.stdout).unwrap();
    let curl_cmd_split: Vec<&str> = curl_cmd_str.split('\n').collect();

    //get date
    let today_string = get_date();
    let today = today_string.as_str();

    //find today contributions
    let mut find_today = false;
    for i in 0..curl_cmd_split.len() {
        if curl_cmd_split[i].contains(today) {
            find_today = true;
            println!("Congratulation!!");
        }
    }
    if !find_today {
        println!("Commit Not yet");
    }

}
