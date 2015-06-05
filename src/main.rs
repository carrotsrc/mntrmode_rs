extern crate rsnl;
extern crate rsgnl;
extern crate nl80211rs;

use std::env::args;

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

    // allocate a new netlink socket
    let mut nls = rsnl::socket::alloc().unwrap();
    rsgnl::socket::connect(&mut nls);
    let family = rsgnl::controller::resolve(&nls, "nl80211");


    println!("Interface Index: {}", dev_index);


    //rsnl::socket::set_buffer_size(&mut nls, 4096, 4096);
    println!("Family Index: {}", family);

    // connect the socket to generic netlink
    rsgnl::socket::connect(&mut nls);


}
