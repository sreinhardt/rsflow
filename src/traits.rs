// packet\flow headers
pub trait DefaultHeader {
	fn get_version(&self) -> u16;
	fn get_count(&self) -> u16;
	fn get_uptime(&self) -> u32;
	fn get_secs(&self) -> u32;
	fn get_sequence(&self) -> u32;
}
#[allow(non_camel_case_types)]
pub trait v5v6Header {
	fn get_eng_type(&self) -> u8;
	fn get_eng_id(&self) -> u8;
	fn get_sampling(&self) -> u16;
}
pub trait GenericNSecs {
	fn get_nsecs(&self) -> u32;
}

// flow data records
pub trait DefaultFlowRecord {
	fn get_src_addr_4(&self) -> [u8;4];
	fn get_dst_addr_4(&self) -> [u8;4];
	fn get_next_hop(&self) -> [u8;4];
	fn get_snmp_in(&self) -> u16;
	fn get_snmp_out(&self) -> u16;
	fn get_pkt_count(&self) -> u32;
	fn get_byte_count(&self) -> u32;
	fn get_start_time(&self) -> u32;
	fn get_end_time(&self) -> u32;
	fn get_src_port(&self) -> u16;
	fn get_dst_port(&self) -> u16;
	fn get_tcp_flags(&self) -> u8;
	fn get_proto(&self) -> u8;
	fn get_tos(&self) -> u8;
	fn get_src_as(&self) -> u16;
	fn get_dst_as(&self) -> u16;
	fn get_src_mask(&self) -> u8;
	fn get_dst_mask(&self) -> u8;
}
#[allow(non_camel_case_types)]
pub trait v7v8FlowRecord {
	fn get_flags(&self) -> u16;
	fn get_router_sc(&self) -> [u8;4];
}
