use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::{env, fs, thread, time};
use std::io::Write;

const SERIALIZE: &str = "/home/ishan/.local/share/serialize.txt";

fn main() {
    // must store some sort of hashmap of dates and actions to occur on those dates

    /*
     * This program is one component of the automatic scheduler that I am attempting to build
     * it should function as follows:
     * the program runs continuously in the background, created at startup with i3
     * it has a hashmap relating times to bash scripts
     * when the current time is equivalent to the time given by one of the elements of the hashmap
     * the program then runs the corresponding bash script, and then removes that entry from the
     * hashmap
     * it is yet to be decided whether
     */

    let args = env::args().collect::<Vec<_>>();
    

    if args.len() > 1 {
        match &args[1] as &str {
            "add" => add(args),
            "remove" => remove(args),
            "list" => list(),
            "help" => help(),
            _ => println!("No matching argument"),
        }
    } else {
        loop {
            // let contents = serde_json::to_string(&hmap).unwrap();
            // puts the thread to sleep for 59 seconds, this is to ensure the program runs around every
            // minute
            thread::sleep(time::Duration::from_millis(59 * 1000));
            let contents;
            // println!("{contents}");
            match fs::read_to_string(SERIALIZE) {
                Ok(content) => contents = content,
                Err(e) => {
                    println!("Couldn't read the file, RIP");
                    std::process::exit(1) /*maybe notify file could not be opened?*/
                }
            }

            let my_map: HashMap<[u32; 5], &str> = serde_json::from_str(&contents).unwrap();

            let time: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
            
            let daily_time = time.time().to_string();
            let date = time.date().to_string();

            let time_day = convert_date(&date as &str, &daily_time as &str);
            // let
            let script_path = my_map.get(&time_day);
            match script_path {
                Some(path) => execute_script(path),
                None => continue,
            }
            println!("time_day = {time_day:?}");
        }
    }
    // let mut my_map: HashMap<Date, Blob>;

    // loop {
    //     // check the entries file every n units of time:
    //     // weigh scheduling versus relative cost of opening
    //     // a file
    //     // huge problem: if you schedule a job, you need to wait at least n units of time for it to
    //     // be saved
    //     // actually this shouldn't matter as this program should never read from the entries file,
    //     // only write to it
    //
    //     let time = SystemTime::now()
    // }
}

fn execute_script(path: &str) {
    std::process::Command::new("bash").arg(path).output().expect("Couldn't execute command");
}

fn add(args: Vec<String>) {
    // deserialize the hasmap, add the entry to the hashmap, then reserialize it 
    let mut serialized_map = fs::read_to_string(SERIALIZE).expect("Could not read file");
    let mut my_map: HashMap<[u32; 5], &str> = serde_json::from_str(&serialized_map).expect("Couldn't convert");
    my_map.insert(convert_date(&args[2] as &str, &args[3] as &str), &args[4] as &str);
    serialized_map = serde_json::to_string(&my_map).expect("Could not serialize");
    let mut serialize_file = File::options().write(true).open(SERIALIZE).expect("Could not open file");
    serialize_file.write_all((&serialized_map as &str).as_bytes()).expect("Could not write to file");
}

fn remove(args: Vec<String>) {
    // deserialize the hasmap, remove the entry from the hashmap, then reserialize it 
    let mut serialized_map = fs::read_to_string(SERIALIZE).expect("Could not read file");
    let mut my_map: HashMap<[u32; 5], &str> = serde_json::from_str(&serialized_map).expect("Couldn't convert");
    my_map.remove(&convert_date(&args[2] as &str, &args[3] as &str));
    serialized_map = serde_json::to_string(&my_map).expect("Could not serialize");
    let mut serialize_file = File::options().write(true).open(SERIALIZE).expect("Could not open file");
    serialize_file.write_all((&serialized_map as &str).as_bytes()).expect("Could not write to file");     
}

fn convert_date(date: &str, time: &str) -> [u32; 5] { 
    // Split the time on ":" so that you can get the individual seconds, minutes, and hours
    // vec would be: [hour, minutes, seconds]
    // HH:MM:SS.SSSSSSSS...
    let time_vec = time.split(":").collect::<Vec<&str>>();
    let hour = time_vec[0].parse::<u32>().unwrap();
    let minute = time_vec[1].parse::<u32>().unwrap();

    // split on "-", vec would be [year, month, day]
    // input is YYYY-MM-DD 
    let date_vec = date.split("-").collect::<Vec<_>>();
    let year = date_vec[0].parse::<u32>().unwrap();
    let month = date_vec[1].parse::<u32>().unwrap();
    let day = date_vec[2].parse::<u32>().unwrap();
    [minute, hour, day, month, year]
}

fn list() {
    // Read the hashmap from the file and then print it out, maybe want a custom print
    let serialized_map = fs::read_to_string(SERIALIZE).expect("Could not read file");
    let my_map: HashMap<[u32; 5], &str> = serde_json::from_str(&serialized_map).expect("Couldn't convert");
    for (time, script) in my_map {
        println!("Will execute {script} at {}", revert_date(time));
    }
}

fn revert_date(date: [u32; 5]) -> String {
    // date: [minute, hour, day, month, year]
    // returns HH:MM on Month Day, Year
    let mut result = String::new();
    result.push_str(&date[1].to_string() as &str);
    result.push(':');
    result.push_str(&date[0].to_string() as &str);
    result.push_str(" on ");
    result.push_str(&month(date[3]).unwrap() as &str);
    result.push_str(" ");
    result.push_str(&date[2].to_string() as &str);
    result.push_str(", ");
    result.push_str(&date[4].to_string() as &str);
    result
}

fn month(num: u32) -> Option<String> {
    match num {
        1 => Some("Jan".to_string()),
        2 => Some("Feb".to_string()),
        3 => Some("Mar".to_string()),
        4 => Some("Apr".to_string()),
        5 => Some("May".to_string()),
        6 => Some("Jun".to_string()),
        7 => Some("Jul".to_string()),
        8 => Some("Aug".to_string()),
        9 => Some("Sep".to_string()),
        10 => Some("Oct".to_string()),
        11 => Some("Nov".to_string()),
        12 => Some("Dec".to_string()),
        _ => None,
    }
}

fn help() {
    println!(r#"
             This program can schedule tasks to be done at specific times: IT CANNOT SCHEDULE RECURRING TASKS 
             It runs in the background at all times and starts at boot
             It reads from the file stored in/home/ishan/.local/share/serialize.txt 
             every minute, therefore it is possible jobs scheduled less than a minute prior to shutdown may not be read by the program 
             Commands:
             add [YYYY-MM-DD] [HH:SS] [/path/to/script]
                schedules the script at the given path to be exectuted on the specific date at the specific time
             
             remove [YYYY-MM-DD] [HH:SS]
                removes the scheduled job at the specific time
             
             list 
                lists all the scheduled jobs and their execution date 

             help 
                prints out the help screen
             "#)
}
