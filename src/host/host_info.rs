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

use sysinfo::{
    Components, Disks, Networks, System,
};
use prettytable::{Cell, row, Row, Table};

pub fn host_info() {
    let mut table = Table::new();
    table.add_row(row![Fgbc => "host-name", "system-name", "kernel", "architecture", "up-time"]);
    table.add_row(Row::new(vec![
        Cell::new(&System::host_name().unwrap().to_string()),
        Cell::new(&System::name().unwrap().to_string()),
        Cell::new(&System::long_os_version().unwrap().to_string()),
        Cell::new(&System::cpu_arch().unwrap().to_string()),
        Cell::new(&System::uptime().to_string()),
    ]));

    table.printstd()
}