use std::{env, thread};
use std::io::{BufReader, BufReadExt};
use std::process::{Command, Stdio};

fn main() {
    let process = match Command::new("adb").arg("devices").output() {
        Ok(p) => p,
        Err(e) => panic!("hoop {}", e)
    };

    let stdout = String::from_utf8_lossy(&process.stdout);
    let device_serials: Vec<_> = stdout.split("\n")
        .map(|s| s.split("\t").next().unwrap_or(""))
        .filter(|&s| !s.starts_with("List of devices attached"))
        .filter(|&s| s.len() > 0)
        .collect();

    let threads: Vec<_> = device_serials.iter().map(|s| {
        let mut cmd = Command::new("adb");
        cmd.arg("-s").arg(s)
           .stdout(Stdio::capture());
        let serial = String::from_str(s);

        thread::spawn(move || {
            let mut args = env::args();
            args.next().unwrap();

            for a in args {
                cmd.arg(&a);
            }

            println!("Cmd: {:?}", cmd);

            let child = cmd.spawn().unwrap_or_else(|e| {
                panic!("Failed to execute child: {}", e);
            });

            let stdout = child.stdout.unwrap_or_else(|| {
                panic!("Unable to get stdout from stream");
            });

            for line in BufReader::new(stdout).lines() {
                println!("{}: {}", serial, line.unwrap());
            }
        })
    }).collect();

    for thread in threads {
        thread.join();
    }

    println!("Done!");
}
