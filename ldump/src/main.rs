#![allow(unused_imports)]

use std::{
    net::Ipv4Addr,
    env,
};

use pnet:: {
    packet::{
        Packet,
        ethernet::{EtherTypes, EthernetPacket},
        ip::IpNextHeaderProtocols,
        ipv4::{checksum, Ipv4Flags, Ipv4Packet, MutableIpv4Packet},
        tcp::{ipv4_checksum, MutableTcpPacket, TcpPacket, TcpFlags},
    },
    datalink::{
        Channel::Ethernet,
        self,
        NetworkInterface,
    },
    ipnetwork,
};

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            // ipv4 数据包
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        // Tcp协议
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!(
                                "Got a Tcp packet {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    },
                    _ => println!("Ignoring non TCP packet"),
                }
            }
        }
        _ => println!("Ignoring non IPv4 packet"),
    }
}

fn main() {
    let interface_name = env::args().nth(1).unwrap();   // 获取命令行参数
    
    let interfaces = datalink::interfaces();            // 网卡列表
    let interface = interfaces
        .into_iter()
        // 根据接口名称过滤网卡列表
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("Error getting interface"); // 如果找不到匹配的接口
                                            // 输出错误并退出
    
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        // 创建数据链路层通道，用于接收和发送数据
        Ok(Ethernet(tx, rx)) => (tx, rx),   // 如果是以太网通道，则将发送和接收通道付给_tx和rx
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        // 获取收到的包
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            },
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            },
        }
    }

}
