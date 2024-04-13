use prettytable::{Cell, row, Row, Table};
use sysinfo::{
    Components, Disks, Networks, System,
};

// Display processes ID, name na disk usage:
pub fn process_summary() {
    // 创建table
    let mut table = Table::new();
    // 增加header头
    table.add_row(row![Fgbc =>"PID", "STATUS","NAME", "DISK-USAGE", "MEMORY-USAGE", "CMD"]);
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        table.add_row(Row::new(vec![
            Cell::new(&pid.to_string()),
            Cell::new(&process.status().to_string()),
            Cell::new(&process.name()),
            Cell::new(&process.disk_usage().total_read_bytes.to_string()),
            Cell::new(&process.memory().to_string()),
        ]));
    }
    table.printstd()
}