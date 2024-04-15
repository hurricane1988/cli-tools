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