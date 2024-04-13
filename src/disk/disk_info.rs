use psutil::*;
use std::thread;
use std::time::Duration;

pub fn show_disk() {
    let mut disk_io_counters_collector = disk::DiskIoCountersCollector::default();
    let disk_io_counters_per_partition = disk_io_counters_collector
        .disk_io_counters_per_partition()
        .unwrap();
    println!("{:?}", disk_io_counters_per_partition)
}