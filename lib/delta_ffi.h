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

typedef struct BlockHeader BlockHeader;

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

typedef struct {
  BlockHeader *thys;
  /**
   * Получить IDINV
   */
  uint32_t (*id_inv)(const BlockHeader*);
  /**
   * Получить IDFMT
   */
  uint16_t (*id_fmt)(const BlockHeader*);
  /**
   * Получить номер обследования
   */
  uint16_t (*rec_num)(const BlockHeader*);
  /**
   * Получить номер блока
   */
  uint16_t (*blk_num)(const BlockHeader*);
  /**
   * Получить время (ре)старта
   */
  uint64_t (*time)(const BlockHeader*);
} FnBlockHeader;

typedef struct {
  uint8_t id_group;
  uintptr_t chan_num;
  uint8_t *buf;
  uintptr_t buf_sz;
} PointDesc;

typedef struct {
  BlockParser *thys;
  /**
   * Пытается открыть (распарсить) блок в переданной области памяти.
   * Если Ok то блок открыт, иначе см. возвращаемый статус
   */
  BlockParserStatus (*try_open_block)(BlockParser *thys, uint8_t *block_mem, uintptr_t block_mem_sz);
  /**
   * Объект загловока отрытого блока, иначе null
   */
  FnBlockHeader (*get_header)(const BlockParser *thys);
  /**
   * Итерируемся по точкам открытого блока пока Ok
   */
  BlockParserStatus (*iter_point)(BlockParser *thys, const PointDesc **out_point);
  /**
   * Удаляет объект парсера
   */
  BlockParserStatus (*disposee)(BlockParser *thys);
} FnBlockParser;

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
FnBlockParser delta_blockparser_new(void);

/**
 * Загловок отрытого блока, иначе null
 */
FnBlockHeader delta_blockparser_header(void);

void delta_debug_blockparser_print_state(const BlockParser *thiz,
                                         const FnBlockHeader *t);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
