use std::process::Command;

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

    for serial in device_serials {
        println!("{}", String::from_utf8_lossy(&Command::new("adb").arg("shell").arg("ls").output().unwrap().stdout));
    }
}
