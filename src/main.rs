use std::process::Command;
use std::str;

//English -> num
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

fn get_today_contributions() -> Option<String> {
    //get date
    let today_string = get_date();
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
        if curl_cmd_split.contains(&today) {
            let today_contribution = curl_cmd_split[i].to_string();
            return Some(today_contribution);
        }
    }
    None
}

fn main() {
    //get_contibution
    let contributions_string = match get_today_contributions() {
        Some(contributions) => contributions,
        None => String::new(),
    };
    let contributions_str = contributions_string.as_str();

    //today_contribution judge
    if contributions_str.contains("data-count=\"0\"") {
        println!("Congratulation!!");
    } else {
        println!("Commit Not yet");
    }

}
