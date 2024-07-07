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

use std::collections::HashMap;
use std::fmt::format;
use std::str::FromStr;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use prettytable::{Table, Row, Cell, row};

pub fn get_tcp_socket() {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;
     
    /// 获取socket信息
     let sockets_info = match get_sockets_info(af_flags, proto_flags) {
         Ok(info) => info,
         Err(e) => {
             eprintln!("Failed to get socket information: {}", e);
             return;
         }
     };
    /// 创建一个表格
    let mut table = Table::new();
    // 添加表格的标题行
    // 增加header头
    table.add_row(row![Fgbc =>"Index","PID","State","Local-Addr", "Local-Port", "Remote-Addr", "Remote-Port"]);

    // 填写表格数据
    for (index,si) in sockets_info.iter().enumerate() {
        if let ProtocolSocketInfo::Tcp(tcp_si) = &si.protocol_socket_info {
            table.add_row(row![
                index,
                /// 将关联的进程ID转换为字符串并合并为一个以逗号分隔的字符串
                si.associated_pids.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(", "),
                tcp_si.state,
                tcp_si.local_addr,
                tcp_si.local_port,
                tcp_si.remote_addr,
                tcp_si.remote_port,
            ]);
        }
    }
    table.printstd();
}

#[derive(Copy, Clone, Debug, PartialEq)]
// 定义TCP连接的状态过滤器枚举
//
// 该枚举用于精确描述TCP连接在生命周期中的不同状态。每个状态代表了连接在特定时间点的特征。
// 这些状态基于TCP连接的典型状态机，涵盖了从初始化到关闭的所有可能阶段。
pub enum TcpStateFilter {
    // 连接已关闭，没有任何活动
    Closed,
    // 监听状态，等待来自客户端的连接请求
    Listen,
    // 已发送SYN报文，等待对方的SYN+ACK报文
    SynSent,
    // 已收到对方的SYN报文，等待对方的ACK报文
    SynReceived,
    // 连接已建立，可以进行数据传输
    Established,
    // 已发送FIN报文，等待对方的FIN报文
    FinWait1,
    // 已收到对方的FIN报文，等待对方的ACK报文
    FinWait2,
    // 已收到对方的FIN报文，等待本地应用发送FIN报文
    CloseWait,
    // 已发送FIN报文和ACK报文，等待对方的FIN报文
    Closing,
    // 已收到对方的FIN报文和ACK报文，等待最后的ACK报文
    LastAck,
    // 连接已关闭，处于TIME_WAIT状态，等待足够的时间以确保所有报文都已传输完毕
    TimeWait,
    // 删除TCP控制块，表示连接已终止
    DeleteTcb,
    // 未知或不识别的状态
    Unknown,
}


/// 从字符串转换为TcpStateFilter枚举值的实现。
///
/// 该实现允许通过字符串表示法来创建TcpStateFilter枚举的实例。
/// 支持的字符串包括各种TCP连接状态的名称，如"closed"、"listen"等。
///
/// # 参数
/// `state`: 一个字符串，表示TCP连接的状态。
///
/// # 返回值
/// 如果提供的字符串与任何已知的TCP状态匹配，则返回对应的TcpStateFilter枚举值。
/// 否则，返回一个错误结果。
impl FromStr for TcpStateFilter {
    type Err = ();

    fn from_str(state: &str) -> Result<TcpStateFilter, Self::Err> {
        // 将输入字符串转换为小写，以支持大小写不敏感的匹配。
        match state.to_lowercase().as_str() {
            "closed" => Ok(TcpStateFilter::Closed),
            "listen" => Ok(TcpStateFilter::Listen),
            "synsent" => Ok(TcpStateFilter::SynSent),
            "synreceived" => Ok(TcpStateFilter::SynReceived),
            "established" => Ok(TcpStateFilter::Established),
            "closewait" => Ok(TcpStateFilter::CloseWait),
            "finwait1" => Ok(TcpStateFilter::FinWait1),
            "closing" => Ok(TcpStateFilter::Closing),
            "lastack" => Ok(TcpStateFilter::LastAck),
            "finwait2" => Ok(TcpStateFilter::FinWait2),
            "timewait" => Ok(TcpStateFilter::TimeWait),
            // 如果没有匹配的字符串，则返回错误。
            _ => Err(()),
        }
    }
}


/// 显示TCP连接状态表
/// 根据指定的TCP状态过滤条件，显示系统的TCP连接状态表。
/// 参数:
/// - state: TcpStateFilter类型的枚举，用于指定要过滤的TCP状态。
fn tcp_state_table(state: TcpStateFilter) {
    // 定义支持的地址族和协议类型
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    // 尝试获取系统中所有TCP连接的信息
    let sockets_info = match get_sockets_info(af_flags, proto_flags) {
        Ok(info) => info,
        Err(e) => {
            // 如果获取信息失败，则打印错误并退出函数
            eprintln!("Failed to get socket information: {}", e);
            return;
        }
    };

    // 初始化表格用于显示结果
    let mut table = Table::new();
    // 设置表格的标题行
    table.add_row(row![Fgbc =>"Index","PID","State","Local-Addr", "Local-Port", "Remote-Addr", "Remote-Port"]);

    // 遍历所有TCP连接信息
    for (index, si) in sockets_info.iter().enumerate() {
        // 只处理TCP协议的连接信息
        if let ProtocolSocketInfo::Tcp(tcp_si) = &si.protocol_socket_info {
            // 根据过滤条件匹配TCP状态
            let tcp_state = match state {
                TcpStateFilter::Listen => TcpState::Listen,
                TcpStateFilter::Closed => TcpState::Closed,
                TcpStateFilter::CloseWait => TcpState::CloseWait,
                TcpStateFilter::Closing => TcpState::Closing,
                TcpStateFilter::Established => TcpState::Established,
                TcpStateFilter::DeleteTcb => TcpState::DeleteTcb,
                TcpStateFilter::FinWait1 => TcpState::FinWait1,
                TcpStateFilter::FinWait2 => TcpState::FinWait2,
                TcpStateFilter::LastAck => TcpState::LastAck,
                TcpStateFilter::SynReceived => TcpState::SynReceived,
                TcpStateFilter::SynSent => TcpState::SynSent,
                TcpStateFilter::TimeWait => TcpState::TimeWait,
                TcpStateFilter::Unknown => TcpState::Unknown,
            };

            // 如果连接状态匹配，则将该连接的信息添加到表格中
            if tcp_si.state.to_string().to_lowercase().as_str() == tcp_state.to_string().to_lowercase().as_str() {
                table.add_row(row![
                    index,
                    si.associated_pids.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(", "),
                    format!("{:?}", tcp_si.state),
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                ]);
            }
        }
    }
    // 打印表格
    table.printstd();
}

pub fn get_tcp_by_state(state: &str) {
    let tcp_state = TcpStateFilter::from_str(&state).unwrap();
    tcp_state_table(tcp_state)
}

pub fn tcp_state_order_table() {
    // Define the address family and protocol flags
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    // Get the socket information
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("Failed to get socket info");

    // Create a HashMap to store the count of each socket state
    let mut socket_counts = std::collections::HashMap::new();

    // Iterate over the socket information and count the states
    for info in sockets_info {
        let state = match info.protocol_socket_info {
            netstat2::ProtocolSocketInfo::Tcp(tcp_info) => format!("{:?}", tcp_info.state),
            netstat2::ProtocolSocketInfo::Udp(_) => "UDP".to_string(),
        };

        *socket_counts.entry(state).or_insert(0) += 1;
    }
    // Convert the HashMap to a Vec and sort it by the count
    let mut socket_counts_vec: Vec<_> = socket_counts.into_iter().collect();
    socket_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // 初始化表格用于显示结果
    let mut table = Table::new();
    // 设置表格的标题行
    table.add_row(row![Fgbc =>"Index","State","Count"]);

    for (index, (state, count)) in socket_counts_vec.iter().enumerate() {
        table.add_row(row![index, state, count]);
    }

    // Print the table
    table.printstd();
}

pub fn remote_addr_order_table() {
    /// Define the address family and protocol flags
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    // Get the socket information.
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("Failed to get socket info");

    // Create a HashMap to store the count of each remote address.
    let mut remote_add_counts: HashMap<String, u32> = HashMap::new();

    // Iterate over the socket information and count  remote addresses.
    for info in sockets_info {
        match info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => {
                /// Count the remote address
                let remote_addr_str = format!("{}", tcp_info.remote_addr);
                *remote_add_counts.entry(remote_addr_str).or_insert(0) +=1;
            }
            ProtocolSocketInfo::Udp(_) => {
                /// udp has not remote_addr
                return;
            },
        }
    }

    // Convert the HashMaps to Vecs and sort them by the count.
    let mut remote_addr_counts_vec: Vec<_> = remote_add_counts.into_iter().collect();
    remote_addr_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));


    // Create the table for remote addresses
    let mut table = Table::new();
    // 设置表格的标题行
    table.add_row(row![Fgbc =>"Index","Remote-Addr","Connected-Count"]);

    for (index, (remote_addr, count)) in remote_addr_counts_vec.iter().enumerate() {
        table.add_row(row![index, remote_addr, count]);
    };
    table.printstd();
}

pub fn local_port_order_table() {
    /// Define the address family and protocol flags
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    // Get the socket information.
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("Failed to get socket info");

    // Create a HashMap to store the count of each remote address.
    let mut local_port_counts: HashMap<String, u32> = HashMap::new();

    // Iterate over the socket information and count  remote addresses.
    for info in sockets_info {
        match info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => {
                /// Count the remote address
                let local_port_str = format!("{}", tcp_info.local_port);
                *local_port_counts.entry(local_port_str).or_insert(0) +=1;
            }
            ProtocolSocketInfo::Udp(_) => {
            },
        }
    }

    // Convert the HashMaps to Vecs and sort them by the count.
    let mut local_port_counts_vec: Vec<_> = local_port_counts.into_iter().collect();
    local_port_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));


    // Create the table for remote addresses
    let mut table = Table::new();
    // 设置表格的标题行
    table.add_row(row![Fgbc =>"Index","Local-Port","Connected-Count"]);

    for (index, (local_port, count)) in local_port_counts_vec.iter().enumerate() {
        table.add_row(row![index, local_port, count]);
    };
    table.printstd();
}

pub fn remote_addr_port_order_table() {
    /// Define the address family and protocol flags
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    // Get the socket information.
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("Failed to get socket info");

    // Create a HashMap to store the count of connections per remote address and port
    let mut remote_addr_port_counts: HashMap<String, u32> = HashMap::new();

    // Iterate over the socket information and count  remote addresses.
    for info in sockets_info {
        match info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => {
                // Count the remote address and port
                let remote_addr_port_str = format!("{}:{}", tcp_info.remote_addr, tcp_info.remote_port);
                *remote_addr_port_counts.entry(remote_addr_port_str).or_insert(0) += 1;
            }
            ProtocolSocketInfo::Udp(_) => {
            },
        }
    }

    let mut remote_addr_port_counts_vec: Vec<_> = remote_addr_port_counts.into_iter().collect();
    remote_addr_port_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));


    // Create the table for remote addresses
    let mut table = Table::new();
    // 设置表格的标题行
    table.add_row(row![Fgbc =>"Index","Remote-Addr-Port","Connected-Count"]);

    for (index, (remote_addr_port, count)) in remote_addr_port_counts_vec.iter().enumerate() {
        table.add_row(row![index + 1, remote_addr_port, count]);
    }
    table.printstd();
}