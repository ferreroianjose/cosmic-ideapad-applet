use glob::glob;
use std::{fs, io, path::PathBuf, process::Command};

const HELPER_NAME: &str = "ideapad_applet_writer";
const SYSFS_DEV_PATH: &str = "/sys/bus/platform/devices/VPC2004:*/";

fn find_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    glob(SYSFS_DEV_PATH)?
        .next()
        .ok_or("No ideapad kernel module loaded?")?
        .map_err(Into::into)
}

pub fn read_module_param(param: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = find_path()?;
    path.push(param);
    Ok(path)
}

fn read_bool_param(param: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let path = read_module_param(param)?;
    let value = fs::read_to_string(&path)?;
    match value.trim() {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(format!("Invalid value for {param}: {value}").into()),
    }
}

fn read_u8_param(param: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let path = read_module_param(param)?;
    let value = fs::read_to_string(&path)?;
    value.trim().parse::<u8>().map_err(Into::into)
}

pub fn get_camera_power() -> Result<bool, Box<dyn std::error::Error>> {
    read_bool_param("camera_power")
}

pub fn get_conservation_mode() -> Result<bool, Box<dyn std::error::Error>> {
    read_bool_param("conservation_mode")
}

pub fn get_fan_mode() -> Result<u8, Box<dyn std::error::Error>> {
    read_u8_param("fan_mode")
}

pub fn get_fn_lock() -> Result<bool, Box<dyn std::error::Error>> {
    read_bool_param("fn_lock")
}

pub fn get_usb_charging() -> Result<bool, Box<dyn std::error::Error>> {
    read_bool_param("usb_charging")
}

pub fn set_camera_power(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_using_helper("camera_power", value.to_string())
}

pub fn set_conservation_mode(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_using_helper("conservation_mode", value.to_string())
}

pub fn set_fan_mode(value: u8) -> Result<(), Box<dyn std::error::Error>> {
    write_using_helper("fan_mode", value.to_string())
}

pub fn set_fn_lock(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_using_helper("fn_lock", value.to_string())
}

pub fn set_usb_charging(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_using_helper("usb_charging", value.to_string())
}

fn write_using_helper(param: &str, value: String) -> Result<(), Box<dyn std::error::Error>> {
    // relative to current binary, same directory
    let helper_path = std::env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            path.push(HELPER_NAME);
            path
        })
        .ok_or("Helper path error")?;

    let status = Command::new("pkexec")
        .arg(helper_path)
        .arg("set")
        .arg(param)
        .arg(value)
        .status()?;

    match status.code() {
        Some(0) => Ok(()),
        Some(code) => Err(io::Error::other(format!("Helper exited with code {code}")).into()),
        None => Err("Helper terminated by signal".into()),
    }
}
