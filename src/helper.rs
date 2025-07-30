use std::{env, fs, process};
mod ideapad_laptop;
use ideapad_laptop::read_module_param;

fn main() {
    let args = parse_args();
    let param = &args[2];
    let value_str = &args[3];

    let result = match param.as_str() {
        "camera_power" => parse_bool(value_str).and_then(set_camera_power),
        "conservation_mode" => parse_bool(value_str).and_then(set_conservation_mode),
        "fan_mode" => parse_u8(value_str).and_then(set_fan_mode),
        "fn_lock" => parse_bool(value_str).and_then(set_fn_lock),
        "usb_charging" => parse_bool(value_str).and_then(set_usb_charging),
        _ => Err(format!("Unknown parameter: {param}").into()),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "set" {
        eprintln!("Usage: {} set <parameter> <value>", args[0]);
        process::exit(1);
    }
    args
}

fn parse_bool(s: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match s.to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err("Value must be true/false or 1/0".into()),
    }
}

fn parse_u8(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
    s.parse::<u8>().map_err(|e| e.into())
}

pub fn set_camera_power(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_bool_param("camera_power", value)
}

pub fn set_conservation_mode(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_bool_param("conservation_mode", value)
}

pub fn set_fan_mode(value: u8) -> Result<(), Box<dyn std::error::Error>> {
    write_u8_param("fan_mode", value)
}

pub fn set_fn_lock(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_bool_param("fn_lock", value)
}

pub fn set_usb_charging(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    write_bool_param("usb_charging", value)
}

fn write_bool_param(param: &str, value: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = read_module_param(param)?;
    let val_str = if value { "1" } else { "0" };
    fs::write(&path, val_str)?;
    Ok(())
}

fn write_u8_param(param: &str, value: u8) -> Result<(), Box<dyn std::error::Error>> {
    let path = read_module_param(param)?;
    fs::write(&path, value.to_string())?;
    Ok(())
}
