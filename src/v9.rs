use byteorder::{BigEndian,ReadBytesExt,ByteOrder};

#[allow(dead_code)]
const HEADER_SIZE:usize = 20;

// flow header \ packet header
#[allow(dead_code)]
struct FlowHeader {
	version: u16,
	count: u16,
	sys_uptime: u32,
	unix_secs: u32,
	//unix_nsecs: u32,
	sequence: u32,
	//*
	source_id: [u8;4],
}
#[allow(dead_code)]
impl FlowHeader {
	pub fn new(buf: &[u8;HEADER_SIZE]) -> FlowHeader {
		FlowHeader {
			version: u16::from_be(BigEndian::read_u16(&buf[0..1])),
			count: u16::from_be(BigEndian::read_u16(&buf[2..3])),
			sys_uptime: u32::from_be(BigEndian::read_u32(&buf[4..7])),
			unix_secs: u32::from_be(BigEndian::read_u32(&buf[8..11])),
			sequence: u32::from_be(BigEndian::read_u32(&buf[12..15])),
			source_id: [buf[16],buf[17],buf[18],buf[19]],
		}
	}
}

#[allow(dead_code)]
enum FieldType {
	IN_BYTES,	// N || 4
	IN_PKTS,	// N || 4
	FLOWS,	// N
	PROTOCOL,	// 1
	SRC_TOS,	// 1
	TCP_FLAGS,	// 1
	L4_SRC_PORT,	// 2
	IPV4_SRC_ADDR,	// 4
	SRC_MASK,	// 1
	INPUT_SNMP,	// N
	L4_DST_ADDR,	// 2
	IPV4_DST_ADDR,	// 4
	DST_MASK,	// 1
	OUTPUT_SNMP,	// N
	IPV4_NEXT_HOP,	// 4
	SRC_AS,	// N || 2
	DST_AS,	// N || 2
	BGP_IPV4_NEXT_HOP,	// 4
	MUL_DST_PKTS,	// N || 4
	MUL_DST_BYTES,	// N || 4
	LAST_SWITCHED,	// 4
	FIRST_SWITCHED,	// 4
	OUT_BYTES,	// N || 4
	OUT_PKTS,	// N || 4
	MIN_PKT_LNGTH,	// 2
	MAX_PKT_LNGTH,	// 2
	IPV6_SRC_ADDR,	// 16
	IPV6_DST_ADDR,	// 16
	IPV6_SRC_MASK,	// 1
	IPV6_DST_MASK,	// 1
	IPV6_FLOW_LABEL,	// 3
	ICMP_TYPE,	// 2
	MUL_IGMP_TYPE,	// 1
	SAMPLING_INTERVAL,	// 4
	SAMPLING_ALGORITHM,	// 1
	FLOW_ACTIVE_TIMEOUT,	// 2
	FLOW_INACTIVE_TIMEOUT,	// 2
	ENGINE_TYPE,	// 1
	ENGINE_ID,	// 1
	TOTAL_BYTES_EXP,	// N || 4
	TOTAL_PKTS_EXP,	// N || 4
	TOTAL_FLOWS_EXP,	// N || 4
	PROPRIETARY1,
	IPV4_SRC_PREFIX,	// 4
	IPV4_DST_PREFIX,	// 4
	MPLS_TOP_LABEL_TYPE,	// 1
	MPLS_TOP_LABEL_IP_ADDR,	// 4
	FLOW_SAMPLER_ID,	// 1
	FLOW_SAMPLER_MODE,	// 1
	FLOW_SAMPLER_RANDOM_INTERVAL,	// 4
	PROPRIETARY2,
	MIN_TTL,	// 1
	MAX_TTL,	// 1
	IPV4_IDENT,	// 4
	DST_TOS,	// 1
	IN_SRC_MAC,	// 6
	OUT_DST_MAC,	// 6
	SRC_VLAN,	// 2
	DST_VLAN,	// 2
	IP_PROTOCOL_VERSION,	// 1
	DIRECTION,	// 1
	IPV6_NEXT_HOP,	// 16
	BGP_IPV6_NEXT_HOP,	// 16
	IPV6_OPTION_HEADERS,	// 4
	PROPRIETARY3,
	PROPRIETARY4,
	PROPRIETARY5,
	PROPRIETARY6,
	PROPRIETARY7,
	MPLS_LABEL_1,	// 3
	MPLS_LABEL_2,	// 3
	MPLS_LABEL_3,	// 3
	MPLS_LABEL_4,	// 3
	MPLS_LABEL_5,	// 3
	MPLS_LABEL_6,	// 3
	MPLS_LABEL_7,	// 3
	MPLS_LABEL_8,	// 3
	MPLS_LABEL_9,	// 3
	MPLS_LABEL_10,	// 3
	IN_DST_MAC,	// 6
	OUT_SRC_MAC,	// 6
	IF_NAME,	// N
	IF_DESC,	// N
	SAMPLER_NAME,	// N
	IN_PERMANENT_BYTES,	// N
	IN_PERMANENT_PKTS,	// N
	PROPRIETARY8,
}

#[allow(dead_code)]
struct Template {
	id: u16,
	field_count: u16,
	fields: Vec<(u16,u16)>, // (type,len)
}

#[allow(dead_code)]
struct TemplateFlowSet {
	id: u16,	// 0
	length: u16,
	templates: Vec<Template>,
}

#[allow(dead_code)]
struct DataFlowSet {
	id: u16, // > 255
	length: u16,
	records: Vec<u16>,
}

#[allow(dead_code)]
enum ScopeType {
	System,
	Interface,
	LineCard,
	NetFlowCache,
	Template,
}

#[allow(dead_code)]
struct OptionsTemplate {
	flowset_id: u16,	// 1
	length: u16,
	template_id: u16,	// 255 < id < 65535
	option_scope_len: u16,
	option_len: u16,
	scope_fields: Vec<(u16,u16)>, // (type,len) scopetype
	option_fields: Vec<(u16,u16)>, // (type,len) fieldtype
}

#[allow(dead_code)]
struct OptionsDataRecord {
	id: u16,	// > 255
	length: u16,
	records: Vec<u16>,
}

#[allow(dead_code)]
enum FlowSets {
	TemplateFlowSet,
	DataFlowSet,
	OptionsTemplate,
	OptionsDataRecord,
}

// export packet
#[allow(dead_code)]
struct ExportPacket {
	packet_header: FlowHeader,
	flowsets: Vec<FlowSets>,
}
