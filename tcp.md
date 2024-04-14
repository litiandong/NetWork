    

# 网络协议栈模型:

	应用层		(Http Http SSH)
	运输层		(TCP UDP)
	网络层		(IP)
	链路层
	物理层

## 第一次握手:

<p>
**信息:**
127.0.0.1.42478 > 127.0.1.1.13200:
Flags [S], 								// Sync
cksum 0xff30 (incorrect -> 0x4e38),		// 检验和
seq 1025185086, 							// 序列号
win 65495, 								// 窗口大小
options [mss 65495,sackOK,TS val 3550806348 ecr 0,nop,wscale 7], 
length 0

**IP 报文头部:**
0x0000:  4500 003c 		// ipv4 首部长度(5Bytes)数据包长度(0x3c->60B)
0x0004:  4da5 4000 		
0x0008:  4006 ee14 		// 寿命(0x40) 上层协议(0x06 = tcp)
0x000c:  7f00 0001  	// 源IP地址
0x0010:  7f00 0101		// 目标IP地址

**TCP 报文起点:**
0x0014:	 a5ee 3390 		// 源端口 目标端口
0x0018:  3d1b 153e 		// 序列号 
0x001c:  0000 0000  	// 确号(无意义)
0x0020:  a002 ffd7 		// 首部长度(10)标志(Sync)接收窗口(0xffd7->65495)
0x0024:  ff30 0000 		// 检验和 紧急数据指针
**选项 ...**
0x0028:  0204 ffd7 				
0x002c:  0402 080a  
0x0030:  d3a5 014c 
0x0034:  0000 0000 
0x0038:  0103 0307 
**头部长度20 Bytes = 首部长度(10) * 2**
</p>


# 第二次握手

	127.0.1.1.13200 > 127.0.0.1.42478: Flags [S.], cksum 0xff30 (incorrect -> 0x034b), seq 2965178357, ack 1025185087, win 65483, options [mss 65495,sackOK,TS val 1671704209 ecr 3550806348,nop,wscale 7], length 0
	0x0000:  4500 003c 0000 4000 4006 3bba 7f00 0101  E..<..@.@.;.....
	0x0010:  7f00 0001 3390 a5ee b0bd 07f5 3d1b 153f  ....3.......=..?
	0x0020:  a012 ffcb ff30 0000 0204 ffd7 0402 080a  .....0..........
	0x0030:  63a4 2e91 d3a5 014c 0103 0307            c......L....
lo    In  IP (tos 0x0, ttl 64, id 19878, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.0.1.42478 > 127.0.1.1.13200: Flags [.], cksum 0xff28 (incorrect -> 0x2a07), ack 1, win 512, options [nop,nop,TS val 3550806348 ecr 1671704209], length 0
	0x0000:  4500 0034 4da6 4000 4006 ee1b 7f00 0001  E..4M.@.@.......
	0x0010:  7f00 0101 a5ee 3390 3d1b 153f b0bd 07f6  ......3.=..?....
	0x0020:  8010 0200 ff28 0000 0101 080a d3a5 014c  .....(.........L
	0x0030:  63a4 2e91                                c...
lo    In  IP (tos 0x0, ttl 64, id 59863, offset 0, flags [DF], proto TCP (6), length 57)
    127.0.1.1.13200 > 127.0.0.1.42478: Flags [P.], cksum 0xff2d (incorrect -> 0xe627), seq 1:6, ack 1, win 512, options [nop,nop,TS val 1671704209 ecr 3550806348], length 5
	0x0000:  4500 0039 e9d7 4000 4006 51e5 7f00 0101  E..9..@.@.Q.....
	0x0010:  7f00 0001 3390 a5ee b0bd 07f6 3d1b 153f  ....3.......=..?
	0x0020:  8018 0200 ff2d 0000 0101 080a 63a4 2e91  .....-......c...
	0x0030:  d3a5 014c 6865 6c6c 6f                   ...Lhello
lo    In  IP (tos 0x0, ttl 64, id 19879, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.0.1.42478 > 127.0.1.1.13200: Flags [.], cksum 0xff28 (incorrect -> 0x2a02), ack 6, win 512, options [nop,nop,TS val 3550806348 ecr 1671704209], length 0
	0x0000:  4500 0034 4da7 4000 4006 ee1a 7f00 0001  E..4M.@.@.......
	0x0010:  7f00 0101 a5ee 3390 3d1b 153f b0bd 07fb  ......3.=..?....
	0x0020:  8010 0200 ff28 0000 0101 080a d3a5 014c  .....(.........L
	0x0030:  63a4 2e91                                c...
lo    In  IP (tos 0x0, ttl 64, id 59864, offset 0, flags [DF], proto TCP (6), length 57)
    127.0.1.1.13200 > 127.0.0.1.42478: Flags [P.], cksum 0xff2d (incorrect -> 0xdc18), seq 6:11, ack 1, win 512, options [nop,nop,TS val 1671704209 ecr 3550806348], length 5
	0x0000:  4500 0039 e9d8 4000 4006 51e4 7f00 0101  E..9..@.@.Q.....
	0x0010:  7f00 0001 3390 a5ee b0bd 07fb 3d1b 153f  ....3.......=..?
	0x0020:  8018 0200 ff2d 0000 0101 080a 63a4 2e91  .....-......c...
	0x0030:  d3a5 014c 776f 726c 64                   ...Lworld
lo    In  IP (tos 0x0, ttl 64, id 19880, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.0.1.42478 > 127.0.1.1.13200: Flags [.], cksum 0xff28 (incorrect -> 0x29fd), ack 11, win 512, options [nop,nop,TS val 3550806348 ecr 1671704209], length 0
	0x0000:  4500 0034 4da8 4000 4006 ee19 7f00 0001  E..4M.@.@.......
	0x0010:  7f00 0101 a5ee 3390 3d1b 153f b0bd 0800  ......3.=..?....
	0x0020:  8010 0200 ff28 0000 0101 080a d3a5 014c  .....(.........L
	0x0030:  63a4 2e91                                c...
lo    In  IP (tos 0x0, ttl 64, id 59865, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.1.1.13200 > 127.0.0.1.42478: Flags [F.], cksum 0xff28 (incorrect -> 0x2611), seq 11, ack 1, win 512, options [nop,nop,TS val 1671705212 ecr 3550806348], length 0
	0x0000:  4500 0034 e9d9 4000 4006 51e8 7f00 0101  E..4..@.@.Q.....
	0x0010:  7f00 0001 3390 a5ee b0bd 0800 3d1b 153f  ....3.......=..?
	0x0020:  8011 0200 ff28 0000 0101 080a 63a4 327c  .....(......c.2|
	0x0030:  d3a5 014c                                ...L
lo    In  IP (tos 0x0, ttl 64, id 19881, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.0.1.42478 > 127.0.1.1.13200: Flags [.], cksum 0xff28 (incorrect -> 0x21fa), ack 12, win 512, options [nop,nop,TS val 3550807395 ecr 1671705212], length 0
	0x0000:  4500 0034 4da9 4000 4006 ee18 7f00 0001  E..4M.@.@.......
	0x0010:  7f00 0101 a5ee 3390 3d1b 153f b0bd 0801  ......3.=..?....
	0x0020:  8010 0200 ff28 0000 0101 080a d3a5 0563  .....(.........c
	0x0030:  63a4 327c                                c.2|
lo    In  IP (tos 0x0, ttl 64, id 19882, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.0.1.42478 > 127.0.1.1.13200: Flags [F.], cksum 0xff28 (incorrect -> 0x1e3c), seq 1, ack 12, win 512, options [nop,nop,TS val 3550808352 ecr 1671705212], length 0
	0x0000:  4500 0034 4daa 4000 4006 ee17 7f00 0001  E..4M.@.@.......
	0x0010:  7f00 0101 a5ee 3390 3d1b 153f b0bd 0801  ......3.=..?....
	0x0020:  8011 0200 ff28 0000 0101 080a d3a5 0920  .....(..........
	0x0030:  63a4 327c                                c.2|
lo    In  IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto TCP (6), length 52)
    127.0.1.1.13200 > 127.0.0.1.42478: Flags [.], cksum 0x1a53 (correct), ack 2, win 512, options [nop,nop,TS val 1671706213 ecr 3550808352], length 0
	0x0000:  4500 0034 0000 4000 4006 3bc2 7f00 0101  E..4..@.@.;.....
	0x0010:  7f00 0001 3390 a5ee b0bd 0801 3d1b 1540  ....3.......=..@
	0x0020:  8010 0200 1a53 0000 0101 080a 63a4 3665  .....S......c.6e
	0x0030:  d3a5 0920   
