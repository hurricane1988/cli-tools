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
use crate::user::user_info::list_users;

// 定义一个命令行工具，用于查询用户信息
pub fn user_list() -> Command {
    // 创建一个新的命令名叫做"user"
    Command::new("user")
        // 设置命令的作者
        .author("Hurricane1988")
        // 设置命令的版本号
        .version("v1.0.0")
        // 设置命令的描述信息
        .about("Print the user information.")
        // 添加一个必选参数"users"，可以通过短选项"-u"或长选项"--users"来指定
        .arg(
            Arg::new("user")
                .short('u')
                .long("users")
                .value_name("USERS")
                .help("Input user subcommand.")
                .required(true),
        )
}

// 处理"list"子命令的函数
pub fn subcommand_user_list(matches: &ArgMatches) {
    // 调用list_users函数，实现查询用户信息的功能
    list_users()
}
