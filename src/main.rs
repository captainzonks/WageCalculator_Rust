use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::data::Data;

mod data;

fn main() {
    let mut data: Data = Data {
        wage: 0.0,
        days: 0.0,
        total_hours: 0.0,
        total_tips: 0.0,
        today_hours: 0.0,
        today_tips: 0.0,
    };

    let mut exists = Path::new("data.txt").exists();
    if exists {
        import_data(&mut data);
        ask_to_change_wage(&mut data);
    } else {
        ask_for_wage(&mut data);
    }

    ask_for_hours(&mut data);
    ask_for_tips(&mut data);

    print_pretty_data(data);
    pause();

    if exists {
        fs::remove_file("data.txt").expect("Could not remove file");
    }
    let mut file = OpenOptions::new().write(true).append(true).create(true).open("data.txt");
    write_to_file(data, &mut file.as_mut().unwrap());
}

fn write_to_file(in_data: Data, file: &mut File) {
    file.write(in_data.get_wage().to_string().as_bytes()).expect("Unable to write file");
    file.write("\n".as_bytes()).expect("Unable to write to file");
    file.write(in_data.get_days().to_string().as_bytes()).expect("Unable to write file");
    file.write("\n".as_bytes()).expect("Unable to write to file");
    file.write(in_data.get_total_hours().to_string().as_bytes()).expect("Unable to write file");
    file.write("\n".as_bytes()).expect("Unable to write to file");
    file.write(in_data.get_total_tips().to_string().as_bytes()).expect("Unable to write file");
}

fn ask_for_wage(in_data: &mut Data) {
    let mut line: String = String::new();
    println!("Please enter your wage: ");
    std::io::stdin().read_line(&mut line).expect("Failed to read input.");

    let user_input: f32 = line.trim().parse().unwrap();

    in_data.set_wage(user_input);
}

fn ask_to_change_wage(in_data: &mut Data) {
    let mut change_wage: String = String::from("n");

    println!("Would you like to change your wage from ${}/hr? (Y or N)", in_data.get_wage());
    change_wage.clear();
    std::io::stdin().read_line(&mut change_wage).expect("Failed to read input.");

    match change_wage.trim().to_lowercase().as_str() {
        "y" => {
            ask_for_wage(in_data);
        }
        _ => println!("You did not change your wage.")
    }
}

fn ask_for_hours(in_data: &mut Data) {
    let mut line: String = String::new();
    println!("Please enter how many hours you worked today: ");
    std::io::stdin().read_line(&mut line).expect("Failed to read input.");

    let user_input: f32 = line.trim().parse().unwrap();

    in_data.add_hours(user_input);
    in_data.set_today_hours(user_input);
}

fn ask_for_tips(in_data: &mut Data) {
    let mut line: String = String::new();
    println!("Please enter your tips: ");
    std::io::stdin().read_line(&mut line).expect("Failed to read input.");

    let user_input: f32 = line.trim().parse().unwrap();

    in_data.add_tips(user_input);
    in_data.set_today_tips(user_input);
}

fn import_data(data: &mut Data) {
    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    let mut x = 0;
    for line in reader.lines() {
        match x {
            0 => data.set_wage(line.unwrap().parse::<f32>().unwrap()),
            1 => data.set_days(line.unwrap().parse::<f32>().unwrap()),
            2 => data.set_total_hours_worked(line.unwrap().parse::<f32>().unwrap()),
            3 => data.set_total_tips_earned(line.unwrap().parse::<f32>().unwrap()),
            _ => {}
        }
        x += 1;
    }
}

fn print_pretty_data(in_data: Data) {
    println!("\nWage: ${}", in_data.get_wage());
    println!("Total hours worked: {}", in_data.get_total_hours());
    println!("Earned today (before tax): ${}", in_data.get_total_daily_earned());
    println!("Earned today (after tax): ${}", in_data.get_total_daily_earned_post_tax());
    println!("Total tips (rounded to nearest dollar): ${}", in_data.get_total_tips());
    println!("Tips Average: ${}", in_data.get_average_tips());
    println!("Today's Adjusted Hourly Rate (before tax): ${}", in_data.get_today_average_wage());
    println!("Today's Adjusted Hourly Rate (after tax): ${}", in_data.get_today_average_wage_post_tax());
    println!("Overall Adjusted Hourly Rate (before tax): ${}", in_data.get_overall_average_wage());
    println!("Overall Adjusted Hourly Rate (after tax): ${}", in_data.get_overall_average_wage_post_tax());
}

fn pause() {
    std::io::stdout().write(b"\nPress Enter to continue...").unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut [0]).unwrap();
}