use std::process::Command;

pub struct SSID {
    id: String,
    status: String,
}

impl SSID {
    pub fn new_query() {
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .spawn();
            println!("windows"); // !!!
        } else if cfg!(target_os = "linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .spawn();
            println!("linux"); // !!!
        }
    }
}

#[cfg(test)]
#[test]
fn test_new_query() {
    let ssid = SSID::new_query();
    //assert_eq!(true, true); /// !!!
}
