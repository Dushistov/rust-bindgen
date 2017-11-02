/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: ::std::os::raw::c_float,
    pub baz: ::std::os::raw::c_float,
    pub bazz: ::std::os::raw::c_double,
    pub bazzz: *mut ::std::os::raw::c_double,
    pub complexFloat: __BindgenComplex<::std::os::raw::c_float>,
    pub complexDouble: __BindgenComplex<::std::os::raw::c_double>,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        48usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bar as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).baz as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(baz))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bazz as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bazz))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bazzz as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(bazzz)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).complexFloat as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(complexFloat)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).complexDouble as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(complexDouble)
        )
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
