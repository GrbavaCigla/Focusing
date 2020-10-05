use std::fs::File;

use psutil::process;
use xdg::BaseDirectories;

fn main() -> std::io::Result<()> {
    // let all_processes = process::processes().unwrap();

    // for process in all_processes{
    //     let cur_proc = process.unwrap();
    //     println!("Proc: {}", cur_proc.name().unwrap());
    // }

    let xdg_dirs = xdg::BaseDirectories::with_prefix("focusing").unwrap();
    let config_path = xdg_dirs.place_config_file("config.ini").unwrap();
    
    let config_file = File::create(config_path)?;

    Ok(())
}