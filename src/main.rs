use std::fs::{File, read_to_string};
use std::path::Path;
use std::{thread, time};

use psutil::process;
use xdg::BaseDirectories;
use configparser::ini::Ini;

fn main() -> std::io::Result<()> {

    // Getting config directory and creating config if doesn't exist
    let xdg_dirs = BaseDirectories::with_prefix("focusing").unwrap();
    let config_path = xdg_dirs.place_config_file("config.ini").unwrap();

    if !Path::new(&config_path).exists() {
        File::create(&config_path).unwrap();
    }
    
    // Getting settings from configparser
    let mut config = Ini::new();
    config.read(read_to_string(&config_path).unwrap()).unwrap();

    // configs
    let delay = match config.getuint("general", "delay") {
        Ok(Some(delay)) => delay,
        Ok(None) => 5,
        Err(_) => 5,
    };
    println!("Delay: {}", delay);
    let delay = time::Duration::from_secs(delay);

    loop {
        // getting all the processes
        let all_processes = match process::processes() {
            Ok(proc) => proc,
            Err(_) => {
                println!("Failed to get processes");
                continue
            },
        };

        // Looping through processes to match on list
        for cur_proc in all_processes {
            if let Ok(proc) = cur_proc {
                // Error handling
                let proc_name = match proc.name() {
                    Ok(proc_name) => proc_name,
                    Err(_) => {
                        println!("Failed to get name of process");
                        continue
                    }
                };


            }

        }
        thread::sleep(delay);
    }
}
