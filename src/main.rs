use core::time;
use std::{process::Command, thread};

use sysinfo::{Process, System};

fn main() {
    watch();
}

fn watch() {
    // TODO: put in config
    let process_name = "eldenring.exe";
    let batch_file_path = &[
        "/C",
        "C:/Users/jason/AppData/Roaming/EldenRing/76561198140326014/commit_and_push.bat",
    ];

    let mut process_running = false;

    let mut system = sysinfo::System::new();
    loop {
        system.refresh_all();

        let mut found = process_running;
        for (_pid, process) in system.processes().iter() {
            if process.name().contains(process_name) {
                found = true;
                break;
            }
            found = false;
        }

        println!("Process running: {0}", found);

        if process_running && !found {
            println!("Running file vc commands");
            let output = Command::new("cmd")
                .args(batch_file_path)
                .output()
                .expect("Failed to execute batch command");

            for out in String::from_utf8(output.stdout).iter() {
                println!("{}", out);
            }
        }

        process_running = found;

        let duration = time::Duration::from_secs(10);
        thread::sleep(duration)
    }
}
