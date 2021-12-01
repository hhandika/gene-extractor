use ansi_term::Colour::Yellow;
use chrono::{Local, NaiveTime};
use sysinfo::{System, SystemExt};

fn parse_duration(duration: u64) -> String {
    let sec = (duration % 60) as u32;
    let min = ((duration / 60) % 60) as u32;
    let hours = ((duration / 60) / 60) as u32;
    let time = NaiveTime::from_hms(hours, min, sec);
    time.format("%H:%M:%S").to_string()
}

pub fn print_formatted_duration(duration: u64) {
    let time = parse_duration(duration);
    log::info!("Execution time (HH:MM:SS): {}", time);
}

pub fn system_info() {
    let sysinfo = sysinfo::System::new_all();
    let total_ram = sysinfo.total_memory();
    let gb = 1048576;

    log::info!("{}", Yellow.paint("System Information"));

    log::info!(
        "{:18}: {} {}",
        "Operating system",
        os_name(&sysinfo),
        os_version(&sysinfo)
    );

    log::info!("{:18}: {}", "Kernel version", kernel_version(&sysinfo));
    log::info!("{:18}: {:?}", "Available cores", num_cpus::get_physical());
    log::info!("{:18}: {:?}", "Available threads", num_cpus::get());
    log::info!("{:18}: {} Gb", "Total RAM", total_ram / gb);
    log::info!(
        "{:18}: {}\n",
        "Date and time",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
}

fn os_name(sysinfo: &System) -> String {
    match sysinfo.name() {
        Some(i) => i,
        None => String::from("UNKNOWN"),
    }
}

fn os_version(sysinfo: &System) -> String {
    match sysinfo.os_version() {
        Some(i) => i,
        None => String::from(""),
    }
}

fn kernel_version(sysinfo: &System) -> String {
    match sysinfo.kernel_version() {
        Some(i) => i,
        None => String::from("UNKNOWN"),
    }
}
