use std::process::Command;

#[derive(Debug)]
pub struct SSID {
    id: String,
    status: String,
}

impl SSID {
    pub fn new_query() -> SSID {
        let mut os;
        if cfg!(target_os = "windows") {
            let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("interfaces")
            .spawn();
            os = "windows".to_owned();
        } else if cfg!(target_os = "linux") {
            let output = Command::new("iwconfig")
            .arg("-r")
            .spawn();
            os = "linux".to_owned();
        }
        SSID {
            id: os,
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
