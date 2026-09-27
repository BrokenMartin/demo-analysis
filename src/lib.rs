use std::sync::atomic::AtomicU32;
pub mod base {
    pub mod cheat_analyser_base;
    pub mod demo_handler_base;
}

pub mod algorithms {
    pub mod all_messages;
    pub mod viewangles_180degrees;
    pub mod viewangles_to_csv;
    pub mod write_to_file;

    pub mod teltta {
        pub mod angle_history;
        pub mod backtrack;
        pub mod double_tap;
    }

    pub mod fidoo {
        pub mod auto_backstab;
        pub mod bunnyhop;
        pub mod invalid_equip_region;
        pub mod nospread;
        pub mod psilent4;
        pub mod silent_aim;
    }
    

    
        

    pub mod nocrex {
        pub mod aimsnap;
        pub mod angle_repeat;
        pub mod oob_pitch;
    }
}

pub mod util {
    pub mod helpers;
    pub mod schema_equip_regions;
    pub mod nocrex {
        pub mod jankguard;
    }
}

pub mod lib {
    pub mod algorithm;
    pub mod parameters;
}

pub const MAX_WORKERS: usize = 1;
pub static WORKER_TICKS: [AtomicU32; MAX_WORKERS] = [AtomicU32::new(0); MAX_WORKERS];

pub static SILENT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub static PROGRESS_CURRENT: AtomicU32 = AtomicU32::new(0);
pub static PROGRESS_TOTAL: AtomicU32 = AtomicU32::new(0);

#[macro_export]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        if !crate::SILENT.load(std::sync::atomic::Ordering::Relaxed) {
            println!($($arg)*);
        }
    }
}