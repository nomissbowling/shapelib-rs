/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
pub struct ShapeC__bindgen_vtable(::std::os::raw::c_void);
#[doc = " no more use (only for test)"]
#[repr(C)]
#[derive(Debug)]
pub struct ShapeC {
    pub vtable_: *const ShapeC__bindgen_vtable,
    pub valid: bool,
    pub fname: *mut ::std::os::raw::c_char,
    pub encode: *mut ::std::os::raw::c_char,
    pub hSHP: *mut ::std::os::raw::c_void,
    pub hDBF: *mut ::std::os::raw::c_void,
    pub c: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShapeC() {
    const UNINIT: ::std::mem::MaybeUninit<ShapeC> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ShapeC>(),
        56usize,
        concat!("Size of: ", stringify!(ShapeC))
    );
    assert_eq!(
        ::std::mem::align_of::<ShapeC>(),
        8usize,
        concat!("Alignment of ", stringify!(ShapeC))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valid) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ShapeC),
            "::",
            stringify!(valid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fname) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ShapeC),
            "::",
            stringify!(fname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encode) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ShapeC),
            "::",
            stringify!(encode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hSHP) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ShapeC),
            "::",
            stringify!(hSHP)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hDBF) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ShapeC),
            "::",
            stringify!(hDBF)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize },
        48usize,
        concat!("Offset of field: ", stringify!(ShapeC), "::", stringify!(c))
    );
}
extern "C" {
    #[link_name = "\u{1}?is_valid@ShapeC@@QEAA_NXZ"]
    pub fn ShapeC_is_valid(this: *mut ShapeC) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?dispose@ShapeC@@QEAAXXZ"]
    pub fn ShapeC_dispose(this: *mut ShapeC);
}
extern "C" {
    #[link_name = "\u{1}?disp_record_inf@ShapeC@@QEAAXXZ"]
    pub fn ShapeC_disp_record_inf(this: *mut ShapeC);
}
extern "C" {
    #[link_name = "\u{1}?alloc_gci@ShapeC@@QEAAPEAXXZ"]
    pub fn ShapeC_alloc_gci(this: *mut ShapeC) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}?free_gci@ShapeC@@QEAAXPEAX@Z"]
    pub fn ShapeC_free_gci(this: *mut ShapeC, p_gci: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}?get_shape@ShapeC@@QEAAXPEAX@Z"]
    pub fn ShapeC_get_shape(this: *mut ShapeC, p_gci: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}?from@ShapeC@@SA?AV1@HH@Z"]
    pub fn ShapeC_from(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ShapeC;
}
extern "C" {
    #[link_name = "\u{1}?to_int@ShapeC@@QEAAHXZ"]
    pub fn ShapeC_to_int(this: *mut ShapeC) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}??0ShapeC@@QEAA@PEBD0@Z"]
    pub fn ShapeC_ShapeC(
        this: *mut ShapeC,
        fn_: *const ::std::os::raw::c_char,
        enc: *const ::std::os::raw::c_char,
    );
}
impl ShapeC {
    #[inline]
    pub unsafe fn is_valid(&mut self) -> bool {
        ShapeC_is_valid(self)
    }
    #[inline]
    pub unsafe fn dispose(&mut self) {
        ShapeC_dispose(self)
    }
    #[inline]
    pub unsafe fn disp_record_inf(&mut self) {
        ShapeC_disp_record_inf(self)
    }
    #[inline]
    pub unsafe fn alloc_gci(&mut self) -> *mut ::std::os::raw::c_void {
        ShapeC_alloc_gci(self)
    }
    #[inline]
    pub unsafe fn free_gci(&mut self, p_gci: *mut ::std::os::raw::c_void) {
        ShapeC_free_gci(self, p_gci)
    }
    #[inline]
    pub unsafe fn get_shape(&mut self, p_gci: *mut ::std::os::raw::c_void) {
        ShapeC_get_shape(self, p_gci)
    }
    #[inline]
    pub unsafe fn from(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ShapeC {
        ShapeC_from(a, b)
    }
    #[inline]
    pub unsafe fn to_int(&mut self) -> ::std::os::raw::c_int {
        ShapeC_to_int(self)
    }
    #[inline]
    pub unsafe fn new(
        fn_: *const ::std::os::raw::c_char,
        enc: *const ::std::os::raw::c_char,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        ShapeC_ShapeC(__bindgen_tmp.as_mut_ptr(), fn_, enc);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DShapeC@@QEAAXXZ"]
    pub fn ShapeC_ShapeC_destructor(this: *mut ShapeC);
}
extern "C" {
    #[doc = " test"]
    pub fn c(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " dup str"]
    pub fn sdup(src: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
