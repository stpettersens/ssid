extern crate regex;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct SSID {
    id: String,
    state: String,
    profile: String,
    interface: String,
}

impl SSID {
    pub fn new() -> SSID {
        let mut id = String::new();
        let mut state = String::new();
        let mut profile = String::new();
        let mut interface = String::new();
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .output()
            .expect("failed to execute process");
            let mut p = Regex::new(r"SSID\s*:\s*([A-z0-9_-]+)").unwrap();
            let o = String::from_utf8_lossy(&output.stdout);
            for cap in p.captures_iter(&o) {
                id = cap[1].to_owned();
                break;
            }
            p = Regex::new(r"State\s*:\s*([a-z]+)").unwrap();
            for cap in p.captures_iter(&o) {
                state = cap[1].to_owned();
            }
            p = Regex::new(r"Profile\s*:\s*([A-z0-9_-]+)").unwrap();
            for cap in p.captures_iter(&o) {
                profile = cap[1].to_owned();
            }
            p = Regex::new(r"Name\s*:\s*([A-z0-9_-]+)").unwrap();
            for cap in p.captures_iter(&o) {
                interface = cap[1].to_owned();
            }
        } else if cfg!(target_os = "linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .output()
            .expect("failed to execute process");
            id = "unimplemented".to_owned();
            state = "unimplemented".to_owned();
            profile = "unimplemented".to_owned();
        }
        SSID {
            id: id,
            state: state,
            profile: profile,
            interface: interface,
        }
    }

    pub fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    pub fn get_state(&self) -> String {
        format!("{}", self.state)
    }

    pub fn get_profile(&self) -> String {
        format!("{}", self.profile)
    }

    pub fn get_interface(&self) -> String {
        format!("{}", self.interface)
    }

    pub fn connect(&self, ssid: &str) {
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("connect")
            .arg(&self.profile)
            .arg(ssid)
            .arg(&self.interface)
            .output()
            .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

#[cfg(test)]
#[test]
fn test_new_query() {
    let ssid = SSID::new();
    println!("{:#?}", ssid);
}
