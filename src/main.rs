use chrono;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, fs, thread, time};

const SERIALIZE: &str = "/home/ishan/.local/share/schedule-jobs/serialize.txt";

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
            "clear" => clear(),
            "background" => background(),
            _ => println!("No matching argument"),
        }
    } else {
        help();
    }
}

fn background() {
    loop {
        // let contents = serde_json::to_string(&hmap).unwrap();
        // puts the thread to sleep for 59 seconds, this is to ensure the program runs around every
        // minute
        let my_map: HashMap<[u32; 5], String> = deserialize();

        let script_path = my_map.get(&current_time());
        match script_path {
            Some(path) => execute_script(path),
            None => {}
        }
        thread::sleep(time::Duration::from_millis(59 * 1000));
    }
}

fn current_time() -> [u32; 5] {
    let time: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();

    let daily_time = time.time().to_string();
    let date = time.date().to_string();

    convert_date(&date as &str, &daily_time as &str)
}

fn execute_script(path: &str) {
    let trimmed_path = path.trim();
    let output = std::process::Command::new("bash")
        .arg(trimmed_path)
        .output()
        .expect("Couldn't execute command");
    println!(
        "stdout was \n {:?} \n stderr was \n {:?} ",
        String::from_utf8(output.stdout).expect("Output unable to be parsed to UTF-8"),
        String::from_utf8(output.stderr).expect("Output unable to be parsed to UTF-8")
    );
}

fn add(args: Vec<String>) {
    // deserialize the hasmap, add the entry to the hashmap, then reserialize it
    let mut my_map: HashMap<[u32; 5], String> = deserialize();
    my_map.insert(
        convert_date(&args[2] as &str, &args[3] as &str),
        args[4].clone(),
    );
    serialize(my_map);
    println!(
        "content of the file is {}",
        fs::read_to_string(SERIALIZE).expect("Sadness")
    );
    // serialized_map = serde_json::to_string(&my_map).expect("Could not serialize");
    // let mut serialize_file = File::options()
    //     .write(true)
    //     .open(SERIALIZE)
    //     .expect("Could not open file");
    // serialize_file
    //     .write_all((&serialized_map as &str).as_bytes())
    //     .expect("Could not write to file");
}

fn remove(args: Vec<String>) {
    // deserialize the hasmap, remove the entry from the hashmap, then reserialize it
    let mut my_map: HashMap<[u32; 5], String> = deserialize();
    my_map.remove(&convert_date(&args[2] as &str, &args[3] as &str));
    serialize(my_map);
    // serialized_map = serde_json::to_string(&my_map).expect("Could not serialize");
    // let mut serialize_file = File::options()
    //     .write(true)
    //     .open(SERIALIZE)
    //     .expect("Could not open file");
    // serialize_file
    //     .write_all((&serialized_map as &str).as_bytes())
    //     .expect("Could not write to file");
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
    let my_map: HashMap<[u32; 5], String> = deserialize();
    for (time, script) in my_map {
        println!("Will execute {} at {}", script.trim(), revert_date(time));
    }
    // doesn't list entire map for some reason: MAJOR ERROR
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
    println!(
        r#"
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
             "#
    )
}

fn serialize(map: HashMap<[u32; 5], String>) {
    let mut to_write = File::options()
        .write(true)
        .truncate(true)
        .open(SERIALIZE)
        .expect("File opening for writing serialized map failed");
    let _ = to_write.write_all(b"");
    for (array, path_to_script) in map {
        let _ = to_write.write_all(b"[");
        for i in array {
            let _ = to_write.write_all((&i.to_string() as &str).as_bytes());
            let _ = to_write.write_all(b", ");
        }
        let _ = to_write.write_all(b"]");
        let _ = to_write.write_all(b" = ");
        let _ = to_write.write_all((&path_to_script as &str).as_bytes());
        let _ = to_write.write_all(b"\n");
    }
}

fn deserialize() -> HashMap<[u32; 5], String> {
    let mut map = HashMap::new();
    // format is
    // [num1, num2, num3, num4, num5] = String
    if let Ok(lines) = read_lines(SERIALIZE) {
        for line in lines {
            let mut path_to_script = String::new();
            if let Ok(line_result) = line {
                let mut array = String::new();
                let mut found = false;
                for character in line_result.chars() {
                    if character == '=' && found == false {
                        found = true;
                        continue;
                    }
                    if found == false {
                        array.push(character);
                    } else {
                        path_to_script.push(character);
                    }
                }
                // key = "[u32, u32, u32, u32, u32]"
                // value = "path/to/file"

                let arr = get_array(&array[1..array.len() - 1]);
                map.insert(arr, path_to_script.clone());
            }
        }
    }
    map
}

fn clear() {
    let map = deserialize();
    let mut new_map = HashMap::new();
    for (time, script) in map {
        if after(time, current_time()) {
            new_map.insert(time, script);
        }
    }
    serialize(new_map);
}

fn after(new_time: [u32; 5], current_time: [u32; 5]) -> bool {
    if new_time[4] < current_time[4] {
        return false;
    } else if new_time[3] < current_time[3] {
        return false;
    } else if new_time[2] < current_time[2] {
        return false;
    } else if new_time[1] < current_time[1] {
        return false;
    } else if new_time[0] < current_time[0] {
        return false;
    } 
    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_array(array: &str) -> [u32; 5] {
    let arr_from_string = array.split(',').collect::<Vec<_>>();
    let mut arr_of_ints: [u32; 5] = [0, 0, 0, 0, 0];
    for i in 0..5 {
        arr_of_ints[i] = arr_from_string[i]
            .trim()
            .parse::<u32>()
            .expect("Couldn't parse int");
    }
    arr_of_ints
}
