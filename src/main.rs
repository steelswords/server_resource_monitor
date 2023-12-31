use sys_info;
use std::time::Duration;
use std::process::Command;
#[macro_use] extern crate rocket;

fn get_cpu_temperature() -> Result<String, String>
{
    let mut command = Command::new("sensors");
    match command.output() {
        Ok(output) => {
            let stdout_string = String::from_utf8(output.stdout).unwrap();
            Ok(stdout_string)
        },
        Err(_) => Err(String::from("Could not run `sensors`. Is lm-sensors installed?"))
    }
}

fn bytes_to_mebibytes(bytes: u64) -> u64
{
    // Shift right is a cleaner, faster way to / 1024
    bytes >> 10
}

fn get_system_info() -> String {
    let load_average = sys_info::loadavg().expect("Could not get load average");
    let ram_info = sys_info::mem_info().expect("Could not get memory info");

    String::from(
        format!(
r#"CPU cores: {}
CPU speed: {} MHz
Load 1m:   {} %
Load 5m:   {} %
Load 15m:  {} %
-----------------------
Memory
-----------------------
Total Memory: {} MB
Free:         {} MB
-----------------------
Total processes: {}

==================
Uptime: {}
{}

"#,
    sys_info::cpu_num().unwrap(),
    sys_info::cpu_speed().unwrap(),
    load_average.one,
    load_average.five,
    load_average.fifteen,
    // Memory Info
    bytes_to_mebibytes(ram_info.total),
    bytes_to_mebibytes(ram_info.free),

    sys_info::proc_total().expect("Could not get number of procs"),
    timeval_to_string(sys_info::boottime().expect("Could not get boottime")),
    get_cpu_temperature().expect("Could not get CPU temperature"),

))
}

fn timeval_to_string(time: libc::timeval) -> String {
    let duration = Duration::new(time.tv_sec as u64, time.tv_usec as u32 * 1000);
    let days = duration.as_secs() / 3600 / 24;
    format!(
        "{:02} days {:02}:{:02}:{:02}.{:03}",
        days,
        duration.as_secs() / 3600 - 24 * days, //hours
        (duration.as_secs() % 3600) / 60,
        duration.as_secs() % 60,
        duration.subsec_millis()
    )
}

#[get("/")]
fn index() -> String {
    get_system_info()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
