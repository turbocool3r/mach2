//! This module corresponds to `mach/port.h`

use vm_types::{integer_t, natural_t};

pub type mach_port_name_t = natural_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct ipc_port;

pub type ipc_port_t = *mut ipc_port;

pub type mach_port_t = ::libc::c_uint;

pub const MACH_PORT_NULL: mach_port_t = 0;
pub const MACH_PORT_DEAD: mach_port_t = !0;

pub type mach_port_right_t = natural_t;

pub const MACH_PORT_RIGHT_SEND: mach_port_right_t = 0;
pub const MACH_PORT_RIGHT_RECEIVE: mach_port_right_t = 1;
pub const MACH_PORT_RIGHT_SEND_ONCE: mach_port_right_t = 2;
pub const MACH_PORT_RIGHT_PORT_SET: mach_port_right_t = 3;
pub const MACH_PORT_RIGHT_DEAD_NAME: mach_port_right_t = 4;
pub const MACH_PORT_RIGHT_LABELH: mach_port_right_t = 5;
pub const MACH_PORT_RIGHT_NUMBER: mach_port_right_t = 6;

pub type mach_port_urefs_t = natural_t;
pub type mach_port_delta_t = integer_t;

pub type mach_port_seqno_t = natural_t;
pub type mach_port_mscount_t = natural_t;
pub type mach_port_msgcount_t = natural_t;
pub type mach_port_rights_t = natural_t;
