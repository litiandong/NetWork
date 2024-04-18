use pnet::packet::{
    icmp::{
        echo_reply::EchoReplyPacket,
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    util,
    Packet,
};
use pnet_transport::icmp_packet_iter;
use pnet_transport::TransportChannelType::Layer4;
use pnet_transport::{transport_channel, TransportProtocol};
use rand::random;
use url::Url;
use tokio;
use std::net;
use std::{
    env,
    net::{IpAddr, ToSocketAddrs, SocketAddr},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

const ICMP_SIZE: usize = 64;    // ICMP数据包大小

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 解析命令行参数，获取目标IP地址
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lping target_ip");
    }
    let url_str: &String = &args[1];
    let mut ip: IpAddr = "0.0.0.0".parse().unwrap();
    println!("url str: {}", url_str);
    let mut address =  tokio::net::lookup_host((url_str.clone(), 80 as u16))
        .await
        .expect("error of lookup_host");
    if let Some(addr) = address.next() {    
        println!("addr: {}", addr);
        ip = addr.ip();
    }

    println!("icmp echo request to target ip: {:#?}", ip);
    
    // 创建传输通道（用于发送和接收ICMP数据包)
    // 确定协议 并且创建数据包通道 tx 为发送通道, rx为接收通道
    let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => return Err(e.into()),
    };

    // 将接收通道转换为迭代器，用于处理接收到的ICMP数据包
    // 将rx接收到的数据包转换为iterator
    let mut iter = icmp_packet_iter(&mut rx);

    loop {
        let mut icmp_header: [u8; ICMP_SIZE] = [0; ICMP_SIZE];
        let icmp_packet = create_icmp_packet(&mut icmp_header);
        // println!("icmp_packet: {:?}", icmp_packet);
        let timer = Arc::new(RwLock::new(Instant::now()));
        // 发送ICMP数据包
        tx.send_to(icmp_packet, ip)?;
        
        match iter.next() {
            Ok((packet, addr)) => match EchoReplyPacket::new(packet.packet()) {
                Some(_echo_reply) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        let start_time = timer.read().unwrap();
                        let rtt = Instant::now().duration_since(*start_time);
                        println!(
                            "ICMP EchoReply received from {:?}: {:?}, Time: {:?}",
                            addr,
                            packet.get_icmp_type(),
                            rtt
                        );
                    } else {
                        println!(
                            "ICMP type other than reply (0) received from {:?}: {:?}",
                            addr,
                            packet.get_icmp_type()
                        );
                    }
                },
                None => {},
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            },
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    //Ok(())
}

// 创建icmp EchoRequest 数据包
fn create_icmp_packet<'a>(icmp_header: &'a mut [u8]) -> MutableEchoRequestPacket<'a> {
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);
    icmp_packet
}

