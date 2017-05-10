extern crate regex;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct SSID {
    id: String,
    status: String,
}

impl SSID {
    pub fn new_query() -> SSID {
        let mut id = String::new();
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .output()
            .expect("failed to execute process");
            let p = Regex::new(r"SSID : ([A-zaz0-9-_]+)$").unwrap();
            for cap in p.captures_iter(&String::from_utf8_lossy(&output.stdout)) {
                id = &cap.at(1).unwrap();
            }
            //println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        } else if cfg!(target_os = "linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .output()
            .expect("failed to execute process");
            id = "unimplemented";
        }
        SSID {
            id: id,
            status: "connected".to_owned(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_new_query() {
    let ssid = SSID::new_query();
    println!("{:?}", ssid);
    //assert_eq!(true, true); /// !!!
}
