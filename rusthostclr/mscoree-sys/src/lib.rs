#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate winapi;

pub use winapi::LPVOID;
use winapi::{HRESULT, GUID};
use winapi::{CLSID, REFCLSID};
use winapi::{IID, REFIID};
use winapi::{BOOL, ULONG, LPCWSTR};

pub const CLSID_CLRMetaHost: CLSID = GUID {
    Data1: 0x9280188d,
    Data2: 0x0e8e,
    Data3: 0x4867,
    Data4: [0xb3,0x0c,0x7f,0xa8,0x38,0x84,0xe8,0xde],
};

pub const IID_ICLRMetaHost: IID = GUID {
    Data1: 0xD332DB9E,
    Data2: 0xB9B3,
    Data3: 0x4125,
    Data4: [0x82,0x07,0xA1,0x48,0x84,0xF5,0x32,0x16],
};

pub const IID_ICLRRuntimeInfo: IID = GUID {
    Data1: 0xBD39D1D2,
    Data2: 0xBA2F,
    Data3: 0x486a,
    Data4: [0x89,0xB0,0xB4,0xB0,0xCB,0x46,0x68,0x91],
};

#[repr(C)]
#[derive(Copy)]
pub struct IUnknownVtbl {
    pub QueryInterface: extern "system" fn(LPVOID, REFIID, *mut LPVOID) -> HRESULT,
    pub AddRef: extern "system" fn(LPVOID) -> ULONG,
    pub Release: extern "system" fn(LPVOID) -> ULONG,
}

#[repr(C)]
#[derive(Copy)]
pub struct ICLRMetaHostVtbl {
    pub __base: IUnknownVtbl,

    // ICLRMetaHost methods
    pub GetRuntime: extern "system" fn(
        This: *mut ICLRMetaHost,
        pwzVersion: LPCWSTR,
        riid: REFIID,
        ppRuntime: *mut LPVOID
    ) -> HRESULT,
}

#[repr(C)]
#[derive(Copy)]
pub struct ICLRRuntimeInfoVtbl {
    pub __base: IUnknownVtbl,

    // ICLRRuntimeInfo methods
    pub __unused: [usize; 6],

    pub GetInterface: extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        rclsid: REFCLSID,
        riid: REFIID,
        ppUnk: *mut LPVOID
    ) -> HRESULT,

    pub IsLoadable: extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pbLoadable: &mut BOOL
    ) -> HRESULT,
}

#[repr(C)]
#[derive(Copy)]
pub struct ICLRMetaHost {
    pub vtbl: *const ICLRMetaHostVtbl,
}

#[repr(C)]
#[derive(Copy)]
pub struct ICLRRuntimeInfo {
    pub vtbl: *const ICLRRuntimeInfoVtbl,
}

extern "system" {
    pub fn CLRCreateInstance(
        clsid: REFCLSID,
        riid: REFIID,
        ppInterface: *mut LPVOID
    ) -> HRESULT;
}