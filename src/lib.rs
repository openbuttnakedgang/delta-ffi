

use std::os::raw::c_char;

#[no_mangle]
#[repr(u8)]
pub enum BlockParserStatus {
    Ok = 0,
    BlockEnd,
    ErrParam,
    ErrDisposed,
}

/// Версия библиотеки
#[no_mangle]
pub extern fn delta_lib_version() -> *const c_char {
    const VERSION: &'static str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const _
}

/// # Низкоуровневый интерфейс для парсинга блока
/// 
/// Использование:  
/// 1. Создать объект blockparser
/// 2. Открыть блок fn try_open_block с областью памяти в которой предположительно находится блок.
///    Размер переданной области памяти должен быть не меньше чем предположительный размер блока.
///    Эта область памяти запоминается парсером, и используется для извлечения заголовка и точек,
///    так что она должна быть валидной на протяжении времени парсинга этого блока.
/// 3. Если получилось открыть блок, (crc сошелся и заголовок валиден),
///    то можно прочесть заголовок: fn header()
/// 4. Далее можно последовательно извлекать точки из открытого блока с помощью fn iter_point.
/// 5. После извлечения последней точки блока (fn iter_point вернет BlockParserStatus_BlockEnd), парсинг блока завершен.
/// 6. Объект парсера можно использовать повторно для парсинга нового блока с помощью fn try_open_block.
/// 7. После окончания работы с объектом blockparser, его нужно удалить - fn dispose.
/// 
#[derive(Debug)]
pub struct BlockParser {
    header: Option<&'static [u8]>,
    block: Option<&'static [u8]>,
    crc_match: bool,
    pos: usize,
}

impl Default for BlockParser {
    fn default() -> Self {
        BlockParser {
            header: None,
            block: None,
            crc_match: false,
            pos: 0,
        }
    }
}

impl Drop for BlockParser {
    fn drop(&mut self) {
        println!("Dropping unpacker");
    }
}

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

/// Создает объект парсера
#[no_mangle]
pub extern fn delta_blockparser_new() -> &'static mut BlockParser {
    let unp = Default::default();
    Box::leak(Box::new(unp))
}

/// Пытается открыть (распарсить) блок в переданной области памяти.
/// Если Ok то блок открыт, иначе см. возвращаемый статус
#[no_mangle]
pub extern fn delta_blockparser_try_open_block(
    thiz: &mut BlockParser,
    block_mem: *mut u8,
    block_mem_sz: usize,
) -> BlockParserStatus {
    if block_mem.is_null() {
        return BlockParserStatus::ErrParam;
    }
    if block_mem_sz < 0x800 {
        return BlockParserStatus::ErrParam;
    }

    //TODO: try parse here
    
    BlockParserStatus::Ok
}

/// Загловок отрытого блока, иначе null
#[no_mangle]
pub extern fn delta_blockparser_header() -> Option<&'static BlockHeader> {
    todo!()
}

#[repr(C)]
#[no_mangle]
pub struct PointDesc {
    pub id_group: u8,
    pub chan_num: usize,
    pub buf: *mut u8,
    pub buf_sz: usize,
}

/// Итерируемся по точкам открытого блока пока Ok
#[no_mangle]
pub extern fn delta_blockparser_iter_point(
    thiz: &mut BlockParser,
    out_point: *mut PointDesc,
) -> BlockParserStatus {
    BlockParserStatus::Ok
}

/// Удаляет объект парсера
#[no_mangle]
pub extern fn delta_blockparser_dispose(
    this_ptr: *mut *mut BlockParser
) -> BlockParserStatus {
    if this_ptr.is_null() {
        return BlockParserStatus::ErrParam;
    }

    let this: *mut BlockParser = unsafe { this_ptr.read() };
    if this.is_null() {
        return BlockParserStatus::ErrDisposed;
    }

    unsafe {
        let p = Box::from_raw(this);
        delta_debug_blockparser_print_state(Some(&p));

        // Set null
        this_ptr.write(std::ptr::null_mut());
    }

    BlockParserStatus::Ok
}

#[no_mangle]
pub extern fn delta_debug_blockparser_print_state(thiz: Option<&BlockParser>) {
    if let Some(unpacker) = thiz {
        println!("{:#?}", unpacker);
    } else {
        println!("NONE Unpacker");
    }
}

