

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

#[repr(C)]
#[no_mangle]
pub struct FnBlockParser<'a> {
    pub thys: &'a mut BlockParser,

    /// Пытается открыть (распарсить) блок в переданной области памяти.
    /// Если Ok то блок открыт, иначе см. возвращаемый статус
    pub try_open_block: fn (
        thys: &mut BlockParser,
        block_mem: *mut u8,
        block_mem_sz: usize,
    ) -> BlockParserStatus,

    /// Объект загловока отрытого блока, иначе null
    pub get_header: fn (thys: &BlockParser) -> FnBlockHeader,

    /// Итерируемся по точкам открытого блока пока Ok
    pub iter_point: fn (
        thys: &mut BlockParser,
        out_point: *mut *const PointDesc,
    ) -> BlockParserStatus,

    /// Удаляет объект парсера
    pub disposee: fn (
        thys: *mut BlockParser
    ) -> BlockParserStatus,
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

#[repr(C, packed)]
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

#[repr(C)]
#[no_mangle]
pub struct FnBlockHeader<'a> {
    pub thys: &'a mut BlockHeader,
    /// Получить IDINV
    pub id_inv: fn (&BlockHeader) -> u32,
    /// Получить IDFMT
    pub id_fmt: fn (&BlockHeader) ->u16,
    /// Получить номер обследования
    pub rec_num: fn (&BlockHeader) ->u16,
    /// Получить номер блока
    pub blk_num: fn (&BlockHeader) ->u16,
    /// Получить время (ре)старта
    pub time: fn (&BlockHeader) ->u64,
}

/// Создает объект парсера
#[no_mangle]
pub extern fn delta_blockparser_new() -> FnBlockParser<'static> {
    //let unp = Default::default();
    //Box::leak(Box::new(unp))
    todo!()
}

/// Пытается открыть (распарсить) блок в переданной области памяти.
/// Если Ok то блок открыт, иначе см. возвращаемый статус
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
pub extern fn delta_blockparser_header() -> FnBlockHeader<'static> {
    static mut TEST_BH: BlockHeader = BlockHeader {
        sign: 0x74,
        id_inv: 0xA5A5A5A5,
        id_fmt: 0xE5E5,
        rec_num: 0x4242,
        blk_num: 0x2424,
        time: 0xB6B6B6B6_B7B7B7B7,
        crc: 0xDADA,
    };
    
    unsafe {
        FnBlockHeader {
            thys: &mut TEST_BH,
            id_inv: |_| TEST_BH.id_inv,
            id_fmt: |_| TEST_BH.id_fmt, 
            rec_num: |_| TEST_BH.rec_num,
            blk_num: |_| TEST_BH.blk_num,
            time: |_| TEST_BH.time,
        }
    }
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
pub extern fn delta_blockparser_iter_point(
    thiz: &mut BlockParser,
    out_point: *mut PointDesc,
) -> BlockParserStatus {
    BlockParserStatus::Ok
}

/// Удаляет объект парсера
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
        //delta_debug_blockparser_print_state(Some(&p));

        // Set null
        this_ptr.write(std::ptr::null_mut());
    }

    BlockParserStatus::Ok
}

#[no_mangle]
pub extern fn delta_debug_blockparser_print_state(thiz: Option<&BlockParser>, t: &FnBlockHeader) {
    if let Some(unpacker) = thiz {
        println!("{:#?}", unpacker);
    } else {
        println!("NONE Unpacker");
    }
}

