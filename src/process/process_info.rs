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