use std::process::Command;

pub struct SSID {
    id: String,
    status: String,
}

impl SSID {
    pub fn new_query() {
        if cfg!("windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .spawn();
        } else if cfg!("linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .spawn();
        }
    }
}

#[cfg(test)]
#[test]
fn test_new_query() {
    let ssid = SSID::new_query();
    //assert_eq!(true, true); /// !!!
}
