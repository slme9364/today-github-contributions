use std::str;

mod date;
mod contributions;

fn main() {
    //get_contibution
    let contributions_string = match contributions::get_today_contributions() {
        Some(contributions) => contributions,
        None => String::new(),
    };
    let contributions_str = contributions_string.as_str();
    //today_contribution judge
    if contributions_str.contains("data-count=\"0\"") {
        println!("Commit Not yet");
    } else {
        println!("Congratulation!!");
    }

}
