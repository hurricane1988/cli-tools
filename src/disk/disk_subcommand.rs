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

use clap::{Arg, ArgMatches, Command};
use crate::disk::disk_info::get_disks;

pub fn disk_sum() -> Command {
    Command::new("disk")
        .author("Hurricane1988")
        .version("v1.0.0")
        .about("Print the disk information.")
        .arg(
            Arg::new("disk")
                .short('d')
                .long("disk")
                .value_name("disk")
                .help("Input disk subcommand.")
                .required(true),
        )
}

pub fn subcommand_disk_sum(matches: &ArgMatches) {
    get_disks()
}