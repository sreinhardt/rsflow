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
