# Monitor Mode Switch
## Demo for libnl and libnl-genl interface for rust

This demonstrates the use of [rsnl](https://github.com/carrotsrc/rsnl) and [rsgnl](https://github.com/carrotsrc/rsgnl).

They are both the beginnings of interfaces (or bindings) to [libnl](http://www.infradead.org/~tgr/libnl/) and libnl-genl, enabling the use of libnl in rust to send netlink messages. Although far from complete, they have reached at least a point of sending messages, but not yet receiving.

#### Demo

This very basic demonstration shows how to switch a wireless device onto monitor mode, set for capturing control frames. There is no error out on the message so if the device does not support monitor mode then it won't tell you. You can check that the device was switched over with `iw [device_name] info` or `ifconfig`.

#### Usage

It needs root privileges to work correctly and the device needs to be brought down/offline.

`sudo ./target/debug/mntmode [device_name]`

e.g.

`sudo ./target/debug/mntmode wlan0`

switching a device back with `iw`

`iw [device_name] set type managed`

#### Dependencies

* libnl-3, libnl-genl-3
* rsnl
* rsgnl
* nl80211rs -- this is a hand converted version of the nl80211.h public interface header

#### License

The demonstration code is CC0



