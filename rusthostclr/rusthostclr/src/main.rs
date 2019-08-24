#![feature(std_misc)]

extern crate "mscoree-sys" as mscoree;

use std::ptr;
use std::mem;
use std::ffi::{AsOsStr};
use std::os::windows::OsStrExt;

fn main() {
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() {
    let meta_host: *mut mscoree::ICLRMetaHost = ptr::null_mut();
    let runtime_info: *mut mscoree::ICLRRuntimeInfo = ptr::null_mut();

    let x = mscoree::CLRCreateInstance(
        &mscoree::CLSID_CLRMetaHost,
        &mscoree::IID_ICLRMetaHost,
        mem::transmute(&meta_host)
    );

    let x = ((*(*meta_host).vtbl).GetRuntime)(
        meta_host,
        to_utf16("v4.0.30319").as_ptr(),
        &mscoree::IID_ICLRRuntimeInfo,
        mem::transmute(&runtime_info)
    );

    let is_loadable = false;
    let x = ((*(*runtime_info).vtbl).IsLoadable)(
        runtime_info,
        mem::transmute(&is_loadable)
    );

    println!("{:?}", is_loadable);
}

fn to_utf16(s: &str) -> Vec<u16> {
    let mut v: Vec<_> = s.as_os_str().encode_wide().collect();
    v.push(0);
    v
}
