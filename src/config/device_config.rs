use std::{fmt, thread, time::Duration};

use cnctd_dialogue::Dialog;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum DeviceType {
    Ios,
    Android,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Ios => "iOS",
            Self::Android => "Android",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Device {
    pub device_type: DeviceType,
    pub device_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfig {
    pub ios: Vec<Device>,
    pub android: Vec<Device>,
    pub default_ios: Option<String>,
    pub default_android: Option<String>
}


impl DeviceConfig {
    pub fn add_device(&mut self) {
        let device_type: DeviceType = Dialog::select("Choose device type", None, None, None);
        match device_type {
            DeviceType::Android => {
                let device_id: String = Dialog::input("Enter Device ID", None, None, None);
                if self.android.iter().any(|android_device| android_device.device_id == device_id) {
                    println!("{}", "This device already exists.".yellow());
                    thread::sleep(Duration::from_secs(2));
                } else {
                    self.android.push(Device { device_type: DeviceType::Android, device_id: device_id.clone() });
                    if self.android.len() == 1 {
                        self.default_android = Some(device_id);
                    }
                }
            }
            DeviceType::Ios => {
                let device_id: String = Dialog::input("Enter Device ID", None, None, None);
                if self.ios.iter().any(|ios_device| ios_device.device_id == device_id) {
                    println!("{}", "This device already exists.".yellow());
                    thread::sleep(Duration::from_secs(2));
                } else {
                    self.ios.push(Device { device_type: DeviceType::Ios, device_id: device_id.clone() });
                    if self.ios.len() == 1 {
                        self.default_ios = Some(device_id);
                    }
                }
            }
        }
    }

    pub fn remove_device(&mut self) {
        let device_type: DeviceType = Dialog::select("Choose device type", None, None, None);
        
        match device_type {
            DeviceType::Android => {
                remove_device_from_list(&mut self.android, &mut self.default_android);
            },
            DeviceType::Ios => {
                remove_device_from_list(&mut self.ios, &mut self.default_ios);
            },
        }
        
    }

    pub fn set_default_device(&mut self, device_type: DeviceType) {
        let devices = match device_type {
            DeviceType::Android => &self.android,
            DeviceType::Ios => &self.ios,
        };
        let device_ids: Vec<&str> = devices.iter().map(|device| device.device_id.as_str()).collect();
        let prompt = "Select default device";
        let selected_device_id = Dialog::select_str(prompt, &device_ids, None, None, None);
        match device_type {
            DeviceType::Android => self.default_android = Some(selected_device_id),
            DeviceType::Ios => self.default_ios = Some(selected_device_id)
        }
    }

    pub fn display_devices(&self) {
        if self.ios.is_empty() { 
            println!("\n{}", "No iOS devices".yellow());
        } else {
            println!("{}", "\niOS".underline());
            for device in &self.ios {
                let is_default = if self.default_ios == Some(device.device_id.clone()) {
                    " (Default)" 
                } else {
                    ""
                };
                println!("Device ID: {}{}", device.device_id, is_default.blue());
            }
        }
        if self.android.is_empty() { 
            println!("\n{}", "No Android devices".yellow());
        } else {
            println!("{}", "\nAndroid".underline());
            for device in &self.android {
                let is_default = if self.default_android == Some(device.device_id.clone()) {
                    " (Default)" 
                } else {
                    ""
                };
                println!("Device ID: {}{}", device.device_id, is_default.blue());
            }
        }
        println!("\n");
    }
}


fn remove_device_from_list(devices: &mut Vec<Device>, default_device: &mut Option<String>) {
    let device_ids: Vec<&str> = devices.iter().map(|device| device.device_id.as_str()).collect();
    let prompt = "Which device would you like to remove?";
    let selected_device_id = Dialog::select_str(prompt, &device_ids, None, None, None);
    
    if let Some(index) = devices.iter().position(|device| device.device_id == selected_device_id) {
        devices.remove(index);
        println!("Removed device: {}", selected_device_id);
        
        if *default_device == Some(selected_device_id.clone()) {
            *default_device = None;
            if !devices.is_empty() {
                *default_device = Some(devices[0].device_id.clone());
            }
        }
    } else {
        println!("Device not found.");
    }
}
