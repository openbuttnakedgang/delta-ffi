
use std::os::raw::c_char;

#[no_mangle]
#[repr(u32)]
pub enum UnpackerError {
    Ok = 0,
    EParams = 1,
    EDelete,
}

#[no_mangle]
pub extern fn delta_lib_version() -> *const c_char {
    const VERSION: &'static str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const _
}

#[derive(Debug)]
pub struct Unpacker {
    header: Option<&'static [u8]>,
    block: Option<&'static [u8]>,
    crc_match: bool,
    pos: usize,
}

impl Default for Unpacker {
    fn default() -> Self {
        Self {
            header: None,
            block: None,
            crc_match: false,
            pos: 0,
        }
    }
}

impl Drop for Unpacker {
    fn drop(&mut self) {
        println!("Dropping unpacker");
    }
}

//#[repr(C)]
//#[no_mangle]
//#[derive(Debug)]
//pub struct CfgChannel {
//    pub id: u8,
//    pub dummy: [u8;3],
//    pub buf: *mut u8,
//    pub sz: usize,
//    pub wr_sz: usize,
//}

#[repr(C)]
#[repr(packed)]
#[no_mangle]
pub struct BlockHeader {
    pub sign: u8,
    pub id_inv: u32,
    pub id_fmt: u16,
    pub rec_num: u16,
    pub blk_num: u16,
    pub time: u64,
    pub crc: u32,
}

#[no_mangle]
pub extern fn unpacker_new() -> &'static mut Unpacker {
    let unp = Default::default();
    Box::leak(Box::new(unp))
}

#[no_mangle]
pub extern fn unblock_try_open(
    block_mem: *mut u8,
    block_mem_sz: usize,
    out_header: *mut BlockHeader,
) -> UnpackerError {
    if block_mem.is_null() {
        return UnpackerError::EParams;
    }
    if block_mem_sz < 0x800 {
        return UnpackerError::EParams;
    }

    //TODO: try parse here
    
    UnpackerError::Ok
}

#[repr(C)]
#[no_mangle]
pub struct PointDesc {
    pub id_group: u8,
    pub chan_num: usize,
    // Для точек можно использовать вырваниваие i32
    // но для меток подойдет только u8
    pub ptr: *mut u8,
    pub sz: usize,
}

#[no_mangle]
pub extern fn unpacker_iter_point(
    thiz: &mut Unpacker,
    point_mem: *mut u8,
    point_mem_sz: usize,
    out_point: *mut PointDesc,
) -> UnpackerError {
    UnpackerError::Ok
}

#[no_mangle]
pub extern fn unpacker_delete(
    this_ptr: *mut *mut Unpacker
) -> UnpackerError {
    if this_ptr.is_null() {
        return UnpackerError::EParams;
    }

    let this: *mut Unpacker = unsafe { this_ptr.read() };
    if this.is_null() {
        return UnpackerError::EDelete;
    }

    unsafe {
        let p = Box::from_raw(this);
        unpacker_print_state(Some(&p));

        // Set null
        this_ptr.write(std::ptr::null_mut());
    }

    UnpackerError::Ok
}

#[no_mangle]
pub extern fn unpacker_print_state(thiz: Option<&Unpacker>) {
    if let Some(unpacker) = thiz {
        println!("{:#?}", unpacker);
    } else {
        println!("NONE Unpacker");
    }
}

