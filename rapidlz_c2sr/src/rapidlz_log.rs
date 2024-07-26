use std::sync::Mutex;
use std::fmt::Display;
use dyn_fmt::AsStrFormatExt;

macro_rules! va_format {
    ($fmt:expr, $args:expr) => {
        $fmt.format($args)
    };
}

macro_rules! rapidlzfilename {
    () => {
        match file!().rfind('/') {
            Some(index) => &file!()[index + 1..],
            None => file!(),
        }
    };
}
pub(crate) use rapidlzfilename;

macro_rules! rapidlz_log {
    ($error_code:expr, $fmt:expr) => {
        rapidlz_log_write($error_code, rapidlzfilename!(), line!() as u16, $fmt, &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr), *) => {
        rapidlz_log_write($error_code, rapidlzfilename!(), line!() as u16, $fmt, &[$(&$args), *]);
    };
}
pub(crate) use rapidlz_log;

pub type RapidlzLogFunc = fn(message: &str, size: usize);

static RAPIDLZ_LOG_FUNC: Mutex<Option<RapidlzLogFunc>> = Mutex::new(None);

pub fn rapidlz_log_write(error_code: usize, file_name: &str, line: u16, fmt: &str, va_args: &[&dyn Display]) {
    let Some(func) = *RAPIDLZ_LOG_FUNC.lock().unwrap() else {
        return;
    };
    let filename = file_name.to_string();
    let basename = if let Some(index) = filename.rfind('/') {
        &filename[index + 1..]
    } else {
        &filename
    };
    let mut output = format!("\n[Rapidlz-Log] File={}, Line={}, Error={}\n", basename, line, error_code);
    output += &va_format!(fmt, va_args);
    func(&output, output.len());
}

pub fn rapidlz_log_register(func: RapidlzLogFunc) {
    *RAPIDLZ_LOG_FUNC.lock().unwrap() = Some(func);
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn rapidlz_log_test(message: &str, size: usize) {
        println!("message: {}, size: {}", message, size);
    }

    #[test]
    fn test_rapidlz_log_no_slice() {
        rapidlz_log_register(rapidlz_log_test);
        let mut ss = "hello";
        let e = &mut ss;
        rapidlz_log!(0, "test rapidlz_log_no_slice! {} {} {} {}", 1, e, 3.14, "world");
    }
}

