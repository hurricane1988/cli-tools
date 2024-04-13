use psutil::*;

pub fn get_pids() {
    let pids = process::pids();
    println!("{:?}", pids)
}