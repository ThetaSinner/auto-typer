use core::config::Config;
use lazy_static::lazy_static;
use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref INPUT: Mutex<Option<core::model::Input>> = Mutex::new(None);
}

#[repr(C)]
pub struct AutoTyper {
    file: *mut c_char,
    start_delay: u8,
    input_delay: f64,
    next_stage: usize,
}

impl Into<Config> for &mut AutoTyper {
    fn into(self) -> Config {
        unsafe {
            Config {
                file: PathBuf::from(CStr::from_ptr(self.file).to_str().unwrap()),
                start_delay: self.start_delay,
                input_delay: self.input_delay,
            }
        }
    }
}

impl AutoTyper {
    #[no_mangle]
    pub extern "C" fn create() -> AutoTyper {
        let c_string = CString::new("").expect("failed to create cstring");
        AutoTyper {
            file: c_string.into_raw(),
            start_delay: 2,
            input_delay: 250.,
            next_stage: 0,
        }
    }

    #[no_mangle]
    pub extern "C" fn set_file(&mut self, file: *mut c_char) {
        self.file = file;
    }

    #[no_mangle]
    pub extern "C" fn set_start_delay(&mut self, start_delay: u8) {
        self.start_delay = start_delay;
    }

    #[no_mangle]
    pub extern "C" fn set_input_delay(&mut self, input_delay: f64) {
        self.input_delay = input_delay;
    }

    #[no_mangle]
    pub extern "C" fn configure(&mut self) {
        let config: Config = self.into();
        let input = core::load_input(&config);
        if let Ok(input) = input {
            let mut input_value = INPUT.lock().unwrap();
            *input_value = Some(input);
            self.next_stage = 0;
            println!("configure successful")
        }
    }

    #[no_mangle]
    pub extern "C" fn has_next(&mut self) -> bool {
        let input = INPUT.lock().unwrap();
        if input.is_some() {
            self.next_stage < input.as_ref().unwrap().stages.len()
        } else {
            false
        }
    }

    #[no_mangle]
    pub extern "C" fn next(&mut self) {
        let config: Config = self.into();

        let input = INPUT.lock().unwrap();
        if input.is_some() {
            sleep(Duration::from_secs(self.start_delay as u64));
            core::run_stage(
                input.as_ref().unwrap().stages.get(self.next_stage).unwrap(),
                &config,
            );
            self.next_stage += 1;
        }
    }

    #[no_mangle]
    pub extern "C" fn print(&self) {
        unsafe {
            println!(
                "self.file = {:?}, self.start_delay = {}, self.input_delay = {}",
                CStr::from_ptr(self.file),
                self.start_delay,
                self.input_delay
            );
        }
    }
}
