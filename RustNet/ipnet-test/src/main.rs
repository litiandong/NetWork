#![allow(unused_variables)]
use ipnet::{
    IpNet, Ipv4Net, Ipv6Net,
};
use std::net::{
    Ipv4Addr, Ipv6Addr
};
use std::str::FromStr;

/// 
///


fn main() -> std::io::Result<()> {
    let _v4 = Ipv4Net::new(Ipv4Addr::new(127, 0, 0, 1), 24).unwrap();
    let _v6 = Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0), 24).unwrap();
    
    let _v4 = Ipv4Net::from_str("127.0.0.1/24").unwrap();
    let _v6 = Ipv6Net::from_str("fd00::/24").unwrap();

    let v4: Ipv4Net = "127.0.0.1/24".parse().unwrap();
    let _v6: Ipv6Net = "fd00::/24".parse().unwrap();

    let _net: IpNet = IpNet::from(v4);
    let net: IpNet = "127.0.0.1/24".parse().unwrap();

    println!("{}, hostmask = {}", net, net.hostmask());
    println!("{}, netmask = {}", net, net.netmask());

    Ok(())
}

