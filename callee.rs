use std::ffi::c_void;
#[repr(align(8))]
#[repr(C)]
pub enum TEST1
{
    A, B, C, D
}

#[repr(align(4))]
#[repr(C)]
pub enum TEST2
{
    A, B, C, D
}

#[no_mangle]
 pub extern "aapcs"  fn func_from_rust1(
     _fun_ptr:  extern "aapcs" fn(*mut c_void) -> c_void,
     _p_arg: *mut c_void,
     _ptos: *mut u8,
     _prio: u8) -> TEST1
{
    return TEST1::A;
}

#[no_mangle]
 pub extern "aapcs"  fn func_from_rust2(
     _fun_ptr:  extern "aapcs" fn(*mut c_void) -> c_void,
     _p_arg: *mut c_void,
     _ptos: *mut u8,
     _prio: u8) -> TEST2
{
    return TEST2::A;
}

#[no_mangle]
 pub extern "aapcs"  fn func_from_rust3(
     _fun_ptr:  extern "aapcs" fn(*mut c_void) -> c_void,
     _p_arg: *mut c_void,
     _ptos: *mut u8,
     _prio: u8) -> u64
{
    return 0;
}

