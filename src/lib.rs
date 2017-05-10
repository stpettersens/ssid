extern crate regex;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct SSID {
    id: String,
    state: String,
}

impl SSID {
    pub fn new_query() -> SSID {
        let mut id = String::new();
        let mut state = String::new();
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .output()
            .expect("failed to execute process");
            let mut p = Regex::new(r"SSID\s*:\s*([A-z0-9]+)").unwrap();
            let o = String::from_utf8_lossy(&output.stdout);
            for cap in p.captures_iter(&o) {
                id = cap[1].to_owned();
                break;
            }
            p = Regex::new(r"State\s*:\s*([a-z]+)").unwrap();
            for cap in p.capture_iter(&o) {
                state = cap[1].to_owned();
            }
        } else if cfg!(target_os = "linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .output()
            .expect("failed to execute process");
            id = "unimplemented".to_owned();
            state = "unimplemented".to_owned();
        }
        SSID {
            id: id,
            state: state,
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
