#[macro_use]
extern crate rsnl;
extern crate rsgnl;
extern crate nl80211rs;

use std::env::args;

type NlDataI32 = rsnl::message::NetlinkData<i32>;

extern "C" { fn if_nametoindex(name: *const str) -> i32; }

fn main() {

    if args().count() != 2 {
        println!("Device unspecified");
        return;
    }
 
    // get the interface name
    let dev_str =  args().last().unwrap();
    let cs: &str = &dev_str;

    // get the interface index
    let dev_index = unsafe { if_nametoindex(cs) };
    if dev_index == 0 {
        println!("Device has no index");
        return;
    }

    // create a new netlink socket
    let mut nls = rsnl::socket::alloc().unwrap();

    // connect the socket to generic netlink
    rsgnl::socket::connect(&mut nls);

    // resolve the netlink controller family id for 802.11 interfaces
    let family = rsgnl::controller::resolve(&nls, "nl80211");
    rsnl::socket::set_buffer_size(&mut nls, 128, 128);

    // connect the socket to generic netlink
    rsgnl::socket::connect(&mut nls);


    // create new netlink messages
    let mut msg = match rsnl::message::alloc() {
        Some(m) => m,
        None => return
    };

    let mut nst = match rsnl::message::alloc() {
        Some(m) => m,
        None => return
    };
    

    // Add netlink header -- message for setting values on a device
    // Port 0 means we are directing the message to the kernel
    rsgnl::message::put(&mut msg, 0,0,family as i32,0,0,nl80211rs::Commands::SetInterface as u8,0);

    // get the interface type enum value for a monitor interface
    let dev_type: i32 = nl80211rs::IfType::Monitor as i32;

    // set the interface to the device index
    NlaPutU32!(&mut msg, nl80211rs::Attrs::IfIndex as i32, &dev_index);

    // set the device to the device type
    NlaPutU32!(&mut msg, nl80211rs::Attrs::IfType as i32, &dev_type);

    // set the monitor mode to capture control frames
    NlaPutFlag!(&mut nst, nl80211rs::MntrFlags::Control as i32);

    // nest the control frame capture mode in the top message
    rsnl::attribute::put_nested(&mut msg, nl80211rs::Attrs::MntrFlags as i32, &nst);

    rsnl::socket::send_auto(&nls, msg);
}
