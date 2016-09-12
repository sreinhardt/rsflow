use std::result;
use std::io::prelude::*;
use byteorder::{BigEndian,ReadBytesExt,ByteOrder};
use bufstream::BufStream;
use v5;
use v6;
use v7;
use v8;
use generic::PacketHeader;

pub type Result<T> = result::Result<T,NFE>;

#[allow(dead_code)]
pub enum NFE {
	GFHerror,
	FillBufErr,
	InvalidPHVer,
}

#[allow(dead_code)]
pub struct NetFlow<S: Read + Write> {
	stream: BufStream<S>,
	counter: usize,
}

#[allow(dead_code)]
impl<S: Read + Write> NetFlow<S> {
	pub fn new(stream: BufStream<S>) -> NetFlow<S> {
		NetFlow {
			stream: stream,
			counter: 0,
		}
	}
	#[allow(unused_assignments)]
	pub fn get_flow_header(&mut self) -> Result<PacketHeader> {
		let mut rst: Result<PacketHeader> = Err(NFE::GFHerror);
		{
			let buf = match self.stream.fill_buf() {
				Ok(b) => b,
				Err(_) => return Err(NFE::FillBufErr),
			};
			rst = match u16::from_be(BigEndian::read_u16(&buf[0..1])) {
				5 => Ok(PacketHeader::from(v5::FlowHeader::new(&buf[0..23]))),
				6 => Ok(PacketHeader::from(v6::FlowHeader::new(&buf[0..23]))),
				7 => Ok(PacketHeader::from(v7::FlowHeader::new(&buf[0..23]))),
				8 => Ok(PacketHeader::from(v8::FlowHeader::new(&buf[0..23]))),
				_ => Err(NFE::InvalidPHVer)
			};
		}
		if rst.is_ok() {
			self.stream.consume(24);
		}
		rst
	}
}
