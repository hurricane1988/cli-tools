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

// 打印当前主机的所有用户
pub fn list_users() {
    // 创建table
    let mut table = Table::new();
    // 增加header头
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
