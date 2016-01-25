macro_rules! generic_header_o {
	() => {
		version: u16::from_be(BigEndian::read_u16(&buf[0..1])),
		count: u16::from_be(BigEndian::read_u16(&buf[2..3])),
		sysUpTime: u32::from_be(BigEndian::read_u32(&buf[4..7])),
		unixSecs: u32::from_be(BigEndian::read_u32(&buf[8..11])),
		sequence: u32::from_be(BigEndian::read_u32(&buf[16..19])),
	}
}
macro_rules! generic_header_fn {
	() => {
		fn get_version(&self) -> u16 {
			self.version
		}
		fn get_count(&self) -> u16 {
			self.count
		}
		fn get_uptime(&self) -> u32 {
			self.sys_uptime
		}
		fn get_secs(&self) -> u32 {
			self.unix_secs
		}
		fn get_sequence(&self) -> u32 {
			self.sequence
		}
	}
}

macro_rules! generic_nsecs_o {
	() => {
		unixnSecs: u32::from_be(BigEndian::read_u32(&buf[12..15])),
	}
}
macro_rules! generic_nsecs_fn {
	() => {
		fn get_nsecs(&self) -> u32 {
			self.unix_nsecs
		}
	}
}

macro_rules! headers_v5_v6_o {
	() => {
		engine_type: u8::from_be(buf[20]),
		engine_id: u8::from_be(buf[21]),
		sampling_int: u16::from_be(BigEndian::read_u16(&buf[22..23])),
	}
}
macro_rules! headers_5_6_fn {
	() => {
		fn get_eng_type(&self) -> u8 {
			self.engine_type
		}
		fn get_eng_id(&self) -> u8 {
			self.engine_id
		}
		fn get_sampling(&self) -> u16 {
			self.sampling_int
		}
	}
}

macro_rules! generic_flowrecord_fn {
	() => {
		fn get_src_addr_4(&self) -> [u8;4] {
			self.ipv4_src_addr
		}
		fn get_dst_addr_4(&self) -> [u8;4] {
			self.ipv4_dst_addr
		}
		fn get_next_hop(&self) -> [u8;4] {
			self.next_hop
		}
		fn get_snmp_in(&self) -> u16 {
			self.snmp_in
		}
		fn get_snmp_out(&self) -> u16 {
			self.snmp_out
		}
		fn get_pkt_count(&self) -> u32 {
			self.pkt_count
		}
		fn get_byte_count(&self) -> u32 {
			self.byte_count
		}
		fn get_start_time(&self) -> u32 {
			self.start_time
		}
		fn get_end_time(&self) -> u32 {
			self.end_time
		}
		fn get_src_port(&self) -> u16 {
			self.src_port
		}
		fn get_dst_port(&self) -> u16 {
			self.dst_port
		}
		fn get_tcp_flags(&self) -> u8 {
			self.tcp_flags
		}
		fn get_proto(&self) -> u8 {
			self.proto
		}
		fn get_tos(&self) -> u8 {
			self.tos
		}
		fn get_src_as(&self) -> u16 {
			self.src_as
		}
		fn get_dst_as(&self) -> u16 {
			self.dst_as
		}
		fn get_src_mask(&self) -> u8 {
			self.src_mask
		}
		fn get_dst_mask(&self) -> u8 {
			self.dst_mask
		}
	}
}
macro_rules! flowrecord_v7_v8_fn {
	() => {
		fn get_flags(&self) -> u16 {
			self.flags
		}
		fn get_router_sc(&self) -> [u8;4] {
			self.router_sc
		}
	}
}
