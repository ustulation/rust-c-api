use std::{slice, mem};

#[repr(C)]
#[derive(Debug)]
pub struct Inner {
    pub e: u8,
}
impl Drop for Inner {
    fn drop(&mut self) {
        // println!("Dropping {:?}", self);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Abc {
    pub ptr: *mut Inner,
    pub len: usize,
    pub arr: [u8; 5],
}
impl Drop for Abc {
    fn drop(&mut self) {
        // println!("Dropping {:?}", self);
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, self.len) };
        let _ = unsafe { Box::from_raw(slice) };
    }
}

#[no_mangle]
pub extern "C" fn foo(o_cb: extern "C" fn(*const Abc)) {
    let mut v = Vec::with_capacity(1000);
    v.push(Inner { e: 10 });
    v.push(Inner { e: 20 });
    v.push(Inner { e: 30 });
    // println!("{:?}", v.as_mut_ptr());

    let mut boxed_slice = v.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();
    let len = boxed_slice.len();
    mem::forget(boxed_slice);

    let abc = Abc {
        ptr: ptr,
        len: len,
        arr: [1, 3, 5, 6, 88],
    };

    // println!("Calling Callback.");
    o_cb(&abc);
    // println!("Callback returned.");
}

#[no_mangle]
pub extern "C" fn get_arr(o_cb: extern "C" fn(*const [u8; 32])) {
    let mut arr = [0u8; 32];
    for i in 0..arr.len() {
        arr[i] = i as u8;
    }
    // println!("Size Rust: {:?} {:?}",
    //          mem::size_of::<[u8; 32]>(),
    //          mem::size_of::<&[u8; 32]>());

    o_cb(&arr);
}

#[no_mangle]
pub extern "C" fn get_arr_wrong(o_cb: extern "C" fn([u8; 32])) {
    let mut arr = [0u8; 32];
    for i in 0..arr.len() {
        arr[i] = i as u8;
    }
    // println!("Size Rust: {:?} {:?}",
    //          mem::size_of::<[u8; 32]>(),
    //          mem::size_of::<&[u8; 32]>());

    o_cb(arr);
}

#[no_mangle]
pub unsafe extern "C" fn print_arr(a: *const [u8; 32]) {
    // println!("Rust: {:?} - {:?}", a, *a);
    println!("Rust: {:?}", *a);
}
