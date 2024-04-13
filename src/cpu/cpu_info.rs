use sysinfo::{Components, Disks, Networks, System, User, Users};
use prettytable::{Table, Row, Cell, row};

// 获取CPU使用率情况
pub fn cpu_usage() {
    // 创建table
    let mut table = Table::new();
    // 增加header头
    table.add_row(row![Fgbc =>"INDEX", "USER", "UID", "GID", "LOGIN"]);

    table.printstd()
}

// 获取CPU核数
pub fn physical_core_count() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut table = Table::new();
    table.add_row(row![Fgbc =>"physical-core-count", sys.cpus().len()]);
    table.printstd()
}