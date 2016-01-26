use traits::{DefaultHeader,v5v6Header,GenericNSecs};
#[allow(unused_imports)] use v5;
#[allow(unused_imports)] use v6;
use v7;
use v8;

#[allow(dead_code)]
pub struct PacketHeader {
	version: u16,
	count: u16,
	sys_uptime: u32,
	unix_secs: u32,
	unix_nsecs: u32,
	sequence: u32,
	//* reserved: [u8;4] // v8/v9
	engine_type: u8,
	engine_id: u8,
	sampling_int: u16,
}
impl<T: DefaultHeader + v5v6Header + GenericNSecs> From<T> for PacketHeader {
	fn from(fh: T) -> PacketHeader {
		PacketHeader {
			version: fh.get_version(),
			count: fh.get_count(),
			sys_uptime: fh.get_uptime(),
			unix_secs: fh.get_secs(),
			unix_nsecs: fh.get_nsecs(),
			sequence: fh.get_sequence(),
			engine_type: fh.get_eng_id(),
			engine_id: fh.get_eng_type(),
			sampling_int: fh.get_sampling(),
		}
	}
}
impl From<v7::FlowHeader> for PacketHeader {
	fn from(fh: v7::FlowHeader) -> PacketHeader {
		PacketHeader {
			version: fh.get_version(),
			count: fh.get_count(),
			sys_uptime: fh.get_uptime(),
			unix_secs: fh.get_secs(),
			unix_nsecs: fh.get_nsecs(),
			sequence: fh.get_sequence(),
			engine_type: 0,
			engine_id: 0,
			sampling_int: 0,
		}
	}
}
impl From<v8::FlowHeader> for PacketHeader {
	fn from(fh: v8::FlowHeader) -> PacketHeader {
		PacketHeader {
			version: fh.get_version(),
			count: fh.get_count(),
			sys_uptime: fh.get_uptime(),
			unix_secs: fh.get_secs(),
			unix_nsecs: fh.get_nsecs(),
			sequence: fh.get_sequence(),
			engine_type: 0,
			engine_id: 0,
			sampling_int: 0,
		}
	}
}
