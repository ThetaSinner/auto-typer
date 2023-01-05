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
    wpm: f64,
    next_stage: usize,
    stage_count: usize,
}

impl Into<Config> for &mut AutoTyper {
    fn into(self) -> Config {
        unsafe {
            Config {
                file: PathBuf::from(CStr::from_ptr(self.file).to_str().unwrap()),
                start_delay: self.start_delay,
                wpm: self.wpm,
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
            wpm: 250.,
            next_stage: 0,
            stage_count: 0,
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
    pub extern "C" fn set_wpm(&mut self, wpm: f64) {
        self.wpm = wpm;
    }

    #[no_mangle]
    pub extern "C" fn configure(&mut self) {
        let config: Config = self.into();
        let input = core::load_input(&config);
        if let Ok(input) = input {
            self.next_stage = 0;
            self.stage_count = input.stages.len();
            let mut input_value = INPUT.lock().unwrap();
            *input_value = Some(input);
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
    pub extern "C" fn skip(&mut self) {
        if self.next_stage + 1 < self.stage_count {
            self.next_stage += 1;
        }
    }

    #[no_mangle]
    pub extern "C" fn previous(&mut self) {
        if self.next_stage > 0 {
            self.next_stage -= 1;
        }
    }

    #[no_mangle]
    pub extern "C" fn print(&self) {
        unsafe {
            println!(
                "self.file = {:?}, self.start_delay = {}, self.input_delay = {}",
                CStr::from_ptr(self.file),
                self.start_delay,
                self.wpm
            );
        }
    }
}
