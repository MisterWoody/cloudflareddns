use chrono::Local;

pub fn log_prefix() -> String {
    let the_time = Local::now();
    the_time.format("%Y-%m-%d %H:%M:%S%:z").to_string()
}