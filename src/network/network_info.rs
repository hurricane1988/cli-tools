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
use sysinfo::{Networks, System};

/**
 * 获取并显示网络接口信息的表格。
 *
 * 此函数使用sysinfo库获取网络接口信息，然后将这些信息格式化并以美观的表格形式在终端中显示。
 * 表格包括以下列：
 * - 接口名称
 * - MAC地址
 * - 已接收的数据包数
 * - 已发送的数据包数
 * - 接收时的错误数
 * - 发送时的错误数
 * - 总接收字节数
 * - 总发送字节数
 *
 * @return 无返回值
 */
pub fn get_networks() {
    // 初始化表格并指定格式
    let mut table = Table::new();
    // table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // 添加表头行
    table.add_row(row![Fgbc =>
        "接口名称",
        "MAC地址",
        "已接收数据包",
        "已发送数据包",
        "接收时错误",
        "发送时错误",
        "总接收量",
        "总发送量"]);

    let networks = Networks::new_with_refreshed_list();

    // 遍历网络接口及其数据，填充表格
    for (interface_name, data) in &networks {
        table.add_row(Row::new(vec![
            Cell::new(interface_name),
            Cell::new(&data.mac_address().to_string()),
            Cell::new(&data.packets_received().to_string()),
            Cell::new(&data.packets_transmitted().to_string()),
            Cell::new(&data.errors_on_received().to_string()),
            Cell::new(&data.errors_on_transmitted().to_string()),
            Cell::new(&data.total_received().to_string()),
            Cell::new(&data.total_transmitted().to_string())
        ]));
    };

    // 在标准输出中打印表格
    table.printstd();
}
