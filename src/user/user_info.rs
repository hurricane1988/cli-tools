// Copyright 2024 Hurricane1988 Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use prettytable::{Table, Row, Cell, row};
use sysinfo::{Uid, User, Users};
use crate::user;

/// 打印当前主机的所有用户
/// 显示系统中所有用户的列表。
///
/// 此函数创建一个表格，包含用户索引、用户名、用户ID、用户组ID和登录名。
/// 它使用`sysinfo`库来获取用户信息，并通过`prettytable-rs`库来格式化和显示信息。
pub fn list_users() {
    // 初始化一个新表格
    // 创建table
    let mut table = Table::new();
    // 添加表格的标题行
    // 增加header头
    table.add_row(row![Fgbc =>"INDEX", "USER", "UID", "GID", "LOGIN"]);

    // 获取系统中所有用户的信息
    let users = sysinfo::Users::new_with_refreshed_list();

    // 遍历用户列表，为每个用户添加一行到表格中
    for (index, user) in users.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()), // 用户索引
            Cell::new(user.name()), // 用户名
            Cell::new(&user.id().to_string()), // 用户ID
            Cell::new(&user.group_id().to_string()), // 用户组ID
        ]));
    }
    // 在标准输出中打印表格
    // Print the table to stdout
    table.printstd()
}
