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


use prettytable::{Cell, format, row, Row, Table};
use sysinfo::{System};


pub fn get_memory() {
    // Initialize the table with a specified format
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Add the header row
    table.add_row(row![Fgbc =>
        "Total-Memory/GiB", "Available-Memory/GiB",
        "Used-Memory/GiB", "Free-Memory/GiB",
        "Total-Swap/GiB", "Used-Swap/GiB", "Free-Swap/GiB"]);

    // Create a new System object and refresh all system information
    let mut sys = System::new_all();
    // Update all information of our system struct.
    sys.refresh_all();

    // Convert memory and swap information from KiB to GiB and add them to table
    table.add_row(Row::new(vec![
        Cell::new(&byte_to_db(sys.total_memory())),
        Cell::new(&byte_to_db(sys.available_memory())),
        Cell::new(&byte_to_db(sys.used_memory())),
        Cell::new(&byte_to_db(sys.free_memory())),
        Cell::new(&byte_to_db(sys.total_swap())),
        Cell::new(&byte_to_db(sys.used_swap())),
        Cell::new(&byte_to_db(sys.free_swap())),
    ]));

    // Print the table to standard output
    table.printstd();
}

/// u64转为f64
fn byte_to_db(value: u64) -> String {
    let result = value as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", result)
}