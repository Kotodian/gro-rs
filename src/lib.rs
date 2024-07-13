use std::{collections::HashMap, net::IpAddr, time::Instant};

use bytes::{BufMut, BytesMut};
#[derive(Clone, Copy, Debug, Hash)]
pub struct FlowKey {
    src_address: IpAddr,
    dst_address: IpAddr,

    src_port: u16,
    dst_port: u16,
}

pub struct Flow {
    next_timeout: Instant,
    last_ack_number: u32,
    buffers: BytesMut,
}

pub struct FlowTable {
    flows: HashMap<FlowKey, Flow>,
}
