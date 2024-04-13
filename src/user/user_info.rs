use prettytable::{Table, Row, Cell, row};

// 打印当前主机的所有用户
pub fn list_users() {
    // 创建table
    let mut table = Table::new();
    // 增加header头
    ///
    table.add_row(row![Fgbc =>"INDEX", "USER", "UID", "GID", "LOGIN"]);
    let users = sysinfo::Users::new_with_refreshed_list();
    for (index,user) in users.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()),
            Cell::new(user.name()),
            Cell::new(&user.id().to_string()),
            Cell::new(&user.group_id().to_string()),
        ]));
    }
    // Print the table to stdout
    table.printstd()
}
