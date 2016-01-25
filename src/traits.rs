pub trait DefaultHeader {
	fn get_version(&self) -> u16;
	fn get_count(&self) -> u16;
	fn get_uptime(&self) -> u32;
	fn get_secs(&self) -> u32;
	fn get_sequence(&self) -> u32;
}
pub trait v5v6Header {
	fn get_eng_type(&self) -> u8;
	fn get_eng_id(&self) -> u8;
	fn get_sampling(&self) -> u16;
}
pub trait GenericNSecs {
	fn get_nsecs(&self) -> u32;
}
