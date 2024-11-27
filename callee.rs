#[repr(align(8))]
#[repr(C)]
pub enum TEST1
{
    A,
    B,
    C,
    D,
}

#[repr(align(4))]
#[repr(C)]
pub enum TEST2
{
    A,
    B,
    C,
    D,
}

#[no_mangle]
pub extern "aapcs" fn func_from_rust1() -> TEST1
{
    return TEST1::C;
}

#[no_mangle]
pub extern "aapcs" fn func_from_rust2() -> TEST2
{
    return TEST2::C;
}

#[no_mangle]
pub extern "aapcs" fn func_from_rust3() -> u64
{
    return 0xfffffffff;
}