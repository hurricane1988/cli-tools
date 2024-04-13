mod cpu;
mod user;
mod process;

use crate::user::user_info::{list_users};
use crate::process::process_info::{process_summary};
use crate::cpu::cpu_info::{cpu_usage, physical_core_count};


fn main() {
    // list_users();
    // process_summary();
    physical_core_count()
}