use std::process::Command;
use std::str;

fn num_month(month_str: &str) -> &str {
    match month_str {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
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

    //get date
    let today_string = get_date();
    let today = today_string.as_str();
    println!("Today: {}", today);

    //find today contributions
    if curl_cmd_str.contains(today) {
        println!("Congratulation!!");
    } else {
        println!("Commit Not yet");
    }

}
