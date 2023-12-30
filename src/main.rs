use sys_info;
use std::time::Duration;
use sensors_sys::*;

/*
 * void SensorData::FetchTemp()
{
    sensors_chip_name const * cn;
    int c = 0;
    while ((cn = sensors_get_detected_chips(0, &c)) != 0) {
        std::cout << "Chip: " << cn->prefix << "/" << cn->path << std::endl;

        sensors_feature const *feat;
        int f = 0;

        while ((feat = sensors_get_features(cn, &f)) != 0) {
            std::cout << f << ": " << feat->name << std::endl;

            sensors_subfeature const *subf;
            int s = 0;

            while ((subf = sensors_get_all_subfeatures(cn, feat, &s)) != 0) {
                std::cout << f << ":" << s << ":" << subf->name
                          << "/" << subf->number << " = ";
                double val;
                if (subf->flags & SENSORS_MODE_R) {
                    int rc = sensors_get_value(cn, subf->number, &val);
                    if (rc < 0) {
                        std::cout << "err: " << rc;
                    } else {
                        std::cout << val;
                    }
                }
                std::cout << std::endl;
            }
        }
    }
}
*/
fn get_cpu_temperature() -> Result<f64, String>
{
    /*
    unsafe {
        if 0 != sensors_init(std::ptr::null_mut())
        {
            return Err("Could not initialize sensors".to_string());
        }
        let mut chip_name: sensors_sys::sensors_chip_name;
        let mut chip_number = 0;
        // Loop over all chips libsensors can find
        while sensors_get_detected_chips(std::ptr::null(), &mut chip_number) != std::ptr::null() {
            println!("Chip: {}/{}", chip_name.prefix as &str, chip_name.path);
            chip_number += 1;
        }
                     



    }
    */
    Ok(0.0)
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
r#"CPU temp: {:.2}
CPU cores: {}
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
"#,
    get_cpu_temperature().expect("Could not get CPU temperature"),
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

))
}

pub fn main() {
    println!("{}", get_system_info());
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
