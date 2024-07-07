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

// TODO: 参考链接：https://github.com/clap-rs/clap/blob/master/examples/tutorial_builder/04_04_custom.md

mod cpu;
mod user;
mod process;
mod disk;
mod host;
mod memory;

mod socket;

mod network;
use clap::{Arg, ArgAction, Command};
use psutil::cpu::cpu_count;
use psutil::host::info;
use serde::Deserialize;
use crate::user::user_info::{list_users};
use crate::process::process_info::{process_summary};
use crate::cpu::cpu_info::{physical_core_count};
use crate::cpu::cpu_subcommand::{cpu_cores, subcommand_cpu_cores};
use crate::host::host_info::{host_info};
use crate::disk::disk_info::{get_disks};
use crate::disk::disk_subcommand::{disk_sum, subcommand_disk_sum};
use crate::host::host_subcommand::{host_sum, subcommand_host_sum};
use crate::memory::mem_info::{get_memory};
use crate::memory::memory_subcommand::{memory_sum, subcommand_memory_sum};
use crate::network::network_info::{get_networks};
use crate::network::network_subcommand::{network_sum, subcommand_network_sum};
use crate::process::process_subcommand::{process_sum, subcommand_process_sum};
use crate::user::user_subcommand::{subcommand_user_list, user_list};
use crate::socket::tcp_socket::{get_tcp_by_state, get_tcp_socket, tcp_state_order_table, remote_addr_order_table, local_port_order_table, remote_addr_port_order_table};

fn main() {

    // get_tcp_socket();
    // get_tcp_by_state("established");

    // tcp_state_order_table();
    // remote_addr_order_table();
    // local_port_order_table();
    remote_addr_port_order_table();

    let matches = Command::new("cli-tool")
        .version("v1.0.0")
        .author("hurricane1988")
        .about("A terminal cli tool to get system information base on selected option")
        .subcommand(cpu_cores())
        .subcommand(user_list())
        .subcommand(process_sum())
        .subcommand(disk_sum())
        .subcommand(host_sum())
        .subcommand(network_sum())
        .subcommand(memory_sum())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("cpu") {
        subcommand_cpu_cores(matches);
    } else if let Some(matches) = matches.subcommand_matches("user"){
        subcommand_user_list(matches);
    } else if let Some(matches) = matches.subcommand_matches("process") {
        subcommand_process_sum(matches);
    } else if let Some(matches) = matches.subcommand_matches("disk") {
        subcommand_disk_sum(matches);
    } else if let Some(matches) = matches.subcommand_matches("host"){
        subcommand_host_sum(matches);
    } else if let Some(matches) = matches.subcommand_matches("memory") {
        subcommand_memory_sum(matches);
    } else if let Some(matches) = matches.subcommand_matches("network") {
        subcommand_network_sum(matches);
    }
}