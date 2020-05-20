#pragma once

/* Generated with cbindgen:0.14.2 */

#include <stdint.h>

enum BlockParserStatus
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  OK = 0,
  BLOCKEND,
  ERRPARAM,
  ERRDISPOSED,
};
#ifndef __cplusplus
typedef uint8_t BlockParserStatus;
#endif // __cplusplus

/**
 * # Низкоуровневый интерфейс для парсинга блока
 *
 * Использование:
 * 1. Создать объект blockparser
 * 2. Открыть блок fn try_open_block с областью памяти в которой предположительно находится блок.
 *    Размер переданной области памяти должен быть не меньше чем предположительный размер блока.
 *    Эта область памяти запоминается парсером, и используется для извлечения заголовка и точек,
 *    так что она должна быть валидной на протяжении времени парсинга этого блока.
 * 3. Если получилось открыть блок, (crc сошелся и заголовок валиден),
 *    то можно прочесть заголовок: fn header()
 * 4. Далее можно последовательно извлекать точки из открытого блока с помощью fn iter_point.
 * 5. После извлечения последней точки блока (fn iter_point вернет BlockParserStatus_BlockEnd), парсинг блока завершен.
 * 6. Объект парсера можно использовать повторно для парсинга нового блока с помощью fn try_open_block.
 * 7. После окончания работы с объектом blockparser, его нужно удалить - fn dispose.
 *
 */
typedef struct BlockParser BlockParser;

typedef struct PACKED {
  uint8_t sign;
  uint32_t id_inv;
  uint16_t id_fmt;
  uint16_t rec_num;
  uint16_t blk_num;
  uint64_t time;
  uint32_t crc;
} 
#ifdef _MSC_VER
#pragma pack(push,1)
BlockHeader
#pragma pack(pop)
#else 
BlockHeader
#endif
;

typedef struct {
  uint8_t id_group;
  uintptr_t chan_num;
  uint8_t *buf;
  uintptr_t buf_sz;
} PointDesc;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Версия библиотеки
 */
const char *delta_lib_version(void);

/**
 * Создает объект парсера
 */
BlockParser *delta_blockparser_new(void);

/**
 * Пытается открыть (распарсить) блок в переданной области памяти.
 * Если Ok то блок открыт, иначе см. возвращаемый статус
 */
BlockParserStatus delta_blockparser_try_open_block(BlockParser *thiz,
                                                   uint8_t *block_mem,
                                                   uintptr_t block_mem_sz);

/**
 * Загловок отрытого блока, иначе null
 */
const 
#ifdef _MSC_VER
#pragma pack(push,1)
BlockHeader
#pragma pack(pop)
#else 
BlockHeader
#endif
 *delta_blockparser_header(void);

/**
 * Итерируемся по точкам открытого блока пока Ok
 */
BlockParserStatus delta_blockparser_iter_point(BlockParser *thiz,
                                               PointDesc *out_point);

/**
 * Удаляет объект парсера
 */
BlockParserStatus delta_blockparser_dispose(BlockParser **this_ptr);

void delta_debug_blockparser_print_state(const BlockParser *thiz);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
