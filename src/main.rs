use core::time;
use ini::Ini;
use std::{path::Path, process::Command, thread};

fn main() {
    watch();
}

fn watch() {
    let mut process_name: &str = &String::new();
    let mut batch_path: &str = &String::new();

    let config = Ini::load_from_file("config.ini")
        .expect("[ERROR] Failed to find ini file. Make sure one exists alongside the executable");

    for (s, p) in config.iter() {
        match s {
            Some(opt) => {
                if opt == "Settings" {
                    if p.contains_key("batch path") {
                        batch_path = &p["batch path"];
                    }
                    if p.contains_key("process name") {
                        process_name = &p["process name"];
                    }
                }
            }
            None => {}
        }
    }

    if process_name.is_empty() || batch_path.is_empty() || !Path::new(batch_path).is_file() {
        panic!(
            "[ERROR] config.ini fields <process name> or <batch path> are either missing or incomplete!"
        );
    }

    let mut process_running_prev = false;

    let mut system = sysinfo::System::new();
    loop {
        system.refresh_all();

        let mut found = process_running_prev;
        for (_pid, process) in system.processes().iter() {
            if process.name().contains(process_name) {
                found = true;
                break;
            }
            found = false;
        }

        println!("[INFO] Process running: {0}", found);

        if process_running_prev && !found {
            println!("[INFO] Running file vc commands");
            let mut path = Path::new(batch_path).to_path_buf();
            path.pop();

            let output = Command::new("cmd")
                .current_dir(path.to_str().unwrap())
                .arg(batch_path)
                .output()
                .expect("Failed to execute batch command");

            for out in String::from_utf8(output.stdout).iter() {
                print!("{}", out);
            }
        }

        process_running_prev = found;

        let duration = time::Duration::from_secs(10);
        thread::sleep(duration)
    }
}
