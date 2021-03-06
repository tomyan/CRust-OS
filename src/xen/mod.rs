pub mod emergency_console;
pub mod ffi;
mod event_channels;
pub mod console;
pub mod xenstore;
pub mod mem;

pub use xen::event_channels::EventChannel;
use core::fmt::Write;
use core::ptr::Unique;
use core::str;
use xen::ffi::start_info::*;
use xen::ffi::hypercalls::*;
use xen::ffi::console::xencons_interface;
use xen::ffi::xenstore::xenstore_domain_interface;

pub use xen::console::STDOUT;
pub use xen::emergency_console::DEBUG;

#[no_mangle]
pub extern fn poweroff() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::poweroff);
}

pub fn crash() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::crash);
}

#[allow(unused_variables, non_shorthand_field_patterns)]
pub unsafe fn initialize(info: StartInfoPage) {
    let StartInfoPage {
        magic:              magic,
        nr_pages:           nr_pages,
        shared_info:        shared_info,
        flags:              flags,
        store_mfn:          mut store_mfn,
        store_evtchn:       store_evtchn,
        console:            Console {
            DomU: DomU {
                mfn:        mut console_mfn,
                evtchn:     console_evtchn
            }
        },
        pt_base:            pt_base,
        nr_pt_frames:       nr_pt_frames,
        mfn_list:           mfn_list,
        mod_start:          mod_start,
        mod_len:            mod_len,
        cmd_line:           cmd_line,
        first_p2m_pfn:      first_p2m_pfn,
        nr_p2m_frames:      nr_p2m_frames
    } = info;
    
    writeln!(DEBUG, "prologue!").unwrap();
    writeln!(DEBUG, "Magic: {}", str::from_utf8(&magic).unwrap_or("ERROR")).unwrap();
    writeln!(DEBUG, "nr_pages: {}", nr_pages).unwrap();

    mem::first_p2m_pfn = *first_p2m_pfn;
    mem::nr_p2m_frames = nr_p2m_frames;
    writeln!(DEBUG, "console::initialize").unwrap();
    let console_ref: &mut xencons_interface = &mut *console_mfn;
    console::initialize(Unique::new(console_ref), EventChannel::new(console_evtchn));
    writeln!(DEBUG, "xen::xenstore::initialize").unwrap();
    let store_ref: &mut xenstore_domain_interface = &mut *store_mfn;
    xenstore::initialize(Unique::new(store_ref), EventChannel::new(store_evtchn));
    writeln!(DEBUG, "end of prologue!").unwrap();
}
