use core::time;
use ini::Ini;
use std::{
    path::Path,
    process::{Command},
    thread,
};

fn main() {
    watch();
}

fn watch() {
    let mut process_name: &str = &String::new();
    let mut batch_path: &str = &String::new();
    let mut poll_timeout: u64 = 10u64;

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
                    if p.contains_key("poll timeout") {
                        poll_timeout = match p["poll timeout"].parse::<u64>() {
                            Ok(t) => t,
                            Err(_) => {
                                println!("[WARN] Failed to parse poll timeout. Using default. Ensure value is non negative and greater than zero.");
                                10u64
                            }
                        }
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
            println!("[INFO] Running {:?}", batch_path);
            let mut path = Path::new(batch_path).to_path_buf();
            path.pop();

            let folder = Path::new(&path);
            if !folder.is_dir() {
                panic!("[ERROR] {:?} is not a directory", path);
            }

            let path_str = path.to_str().unwrap();

            let output = Command::new("cmd")
                .current_dir(path_str)
                .args(["/C", batch_path])
                .output()
                .expect("Failed to run cmd");

            println!("status: {}", output.status);

            for out in String::from_utf8(output.stdout).iter() {
                println!("STDOUT: {}", out);
            }

            for out_err in String::from_utf8(output.stderr).iter() {
                println!("STDERR: {}", out_err);
            }
        }

        process_running_prev = found;

        let duration = time::Duration::from_secs(poll_timeout as u64);
        thread::sleep(duration)
    }
}
