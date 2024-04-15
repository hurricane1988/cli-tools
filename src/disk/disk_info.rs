use prettytable::{Cell, row, Row, Table};
use sysinfo::{Disks};

/// 查询获取disk磁盘列表
pub fn get_disks() {
    // 创建table
    let mut table = Table::new();
    // 增加header头
    table.add_row(row![Fgbc =>"Index","Disk", "Type", "FileSystem","Mounted", "Capability/GiB", "Removable"]);
    let disks = Disks::new_with_refreshed_list();
    for (index, disk) in disks.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()),
            Cell::new(disk.name().to_str().unwrap()),
            Cell::new(&disk.kind().to_string()),
            Cell::new(&disk.file_system().to_str().unwrap()),
            Cell::new(&disk.mount_point().to_str().unwrap()),
            Cell::new(&byte_to_db(disk.available_space())),
            Cell::new(&disk.is_removable().to_string()),
        ]));
    }
    table.printstd()
}

/// 转byte转为GB
fn byte_to_db(value: u64) -> String {
    let result = value as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", result)
}