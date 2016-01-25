use byteorder::{BigEndian,ReadBytesExt,ByteOrder};
use macros;
use traits::{DefaultHeader,v5v6Header,GenericNSecs,DefaultFlowRecord};

#[allow(dead_code)]
const HEADER_SIZE:usize = 24;
#[allow(dead_code)]
const RECORD_SIZE:usize = 52;

#[allow(dead_code)]
struct FlowHeader {
	version: u16,
	count: u16,
	sys_uptime: u32,
	unix_secs: u32,
	unix_nsecs: u32,
	sequence: u32,
	//*
	engine_type: u8,
	engine_id: u8,
	sampling_int: u16,
}
#[allow(dead_code)]
impl FlowHeader {
	fn new(buf: &[u8;HEADER_SIZE]) -> FlowHeader {
		FlowHeader {
			version: u16::from_be(BigEndian::read_u16(&buf[0..1])),
			count: u16::from_be(BigEndian::read_u16(&buf[2..3])),
			sys_uptime: u32::from_be(BigEndian::read_u32(&buf[4..7])),
			unix_secs: u32::from_be(BigEndian::read_u32(&buf[8..11])),
			unix_nsecs: u32::from_be(BigEndian::read_u32(&buf[12..15])),
			sequence: u32::from_be(BigEndian::read_u32(&buf[16..19])),
			engine_type: u8::from_be(buf[20]),
			engine_id: u8::from_be(buf[21]),
			sampling_int: u16::from_be(BigEndian::read_u16(&buf[22..23])),
		}
	}
}
impl DefaultHeader for FlowHeader {
	generic_header_fn!();
}
impl GenericNSecs for FlowHeader {
	generic_nsecs_fn!();
}
impl v5v6Header for FlowHeader {
	headers_5_6_fn!();
}

#[allow(dead_code)]
struct FlowRecord {
	ipv4_src_addr: [u8;4],
	ipv4_dst_addr: [u8;4],
	next_hop: [u8;4],
	snmp_in: u16,
	snmp_out: u16,
	pkt_count: u32,
	byte_count: u32,
	start_time: u32,
	end_time: u32,
	src_port: u16,
	dst_port: u16,
	pad1: u8,
	tcp_flags: u8,
	proto: u8,
	tos: u8,
	src_as: u16,
	dst_as: u16,
	src_mask: u8,
	dst_mask: u8,
	//*
	pad2: [u8;6],
}
impl FlowRecord {
	pub fn new(buf: &[u8;RECORD_SIZE]) -> FlowRecord {
		FlowRecord {
			ipv4_src_addr: [u8::from_be(buf[0]),
							u8::from_be(buf[1]),
							u8::from_be(buf[2]),
							u8::from_be(buf[3]),
							],
		 	ipv4_dst_addr: [u8::from_be(buf[4]),
							u8::from_be(buf[5]),
							u8::from_be(buf[6]),
							u8::from_be(buf[7]),
							],
			next_hop: [u8::from_be(buf[8]),
						u8::from_be(buf[9]),
						u8::from_be(buf[10]),
						u8::from_be(buf[11]),
						],
			snmp_in: u16::from_be(BigEndian::read_u16(&buf[12..13])),
			snmp_out: u16::from_be(BigEndian::read_u16(&buf[14..15])),
			pkt_count: u32::from_be(BigEndian::read_u32(&buf[16..19])),
			byte_count: u32::from_be(BigEndian::read_u32(&buf[20..23])),
			start_time: u32::from_be(BigEndian::read_u32(&buf[24..27])),
			end_time: u32::from_be(BigEndian::read_u32(&buf[28..31])),
			src_port: u16::from_be(BigEndian::read_u16(&buf[32..33])),
			dst_port: u16::from_be(BigEndian::read_u16(&buf[34..35])),
			pad1: u8::from_be(buf[36]),
			tcp_flags: u8::from_be(buf[37]),
			proto: u8::from_be(buf[38]),
			tos: u8::from_be(buf[39]),
			src_as: u16::from_be(BigEndian::read_u16(&buf[40..41])),
			dst_as: u16::from_be(BigEndian::read_u16(&buf[42..43])),
			src_mask: u8::from_be(buf[44]),
			dst_mask: u8::from_be(buf[45]),
			pad2: [u8::from_be(buf[46]),
					u8::from_be(buf[47]),
					u8::from_be(buf[48]),
					u8::from_be(buf[49]),
					u8::from_be(buf[50]),
					u8::from_be(buf[51]),
					],
		}
	}
}
impl DefaultFlowRecord for FlowRecord {
	generic_flowrecord_fn!();
}
