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
use crate::cpu::cpu_info::physical_core_count;

pub fn cpu_cores() -> Command {
    Command::new("cpu")
        .author("Hurricane1988")
        .version("v1.0.0")
        .about("Print the cpu usage information.")
        .arg(
            Arg::new("cores")
                .short('c')
                .long("cores")
                .value_name("CORES")
                .help("Input cores subcommand.")
                .required(true),
        )
}

pub fn subcommand_cpu_cores(matches: &ArgMatches) {
    physical_core_count()
}