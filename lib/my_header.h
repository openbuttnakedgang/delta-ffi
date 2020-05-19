#pragma once

/* Generated with cbindgen:0.14.2 */

#include <stdint.h>

enum UnpackerError
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  OK = 0,
  EPARAMS = 1,
  EDELETE,
};
#ifndef __cplusplus
typedef uint32_t UnpackerError;
#endif // __cplusplus

typedef struct Unpacker Unpacker;

typedef struct PACKED {
  uint8_t sign;
  uint32_t id_inv;
  uint16_t id_fmt;
  uint16_t rec_num;
  uint16_t blk_num;
  uint64_t time;
  uint32_t crc;
} BlockHeader;

typedef struct {
  uint8_t id_group;
  uintptr_t chan_num;
  uint8_t *ptr;
  uintptr_t sz;
} PointDesc;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

const char *delta_lib_version(void);

Unpacker *unpacker_new(void);

UnpackerError unblock_try_open(uint8_t *block_mem,
                               uintptr_t block_mem_sz,
                               BlockHeader *out_header);

UnpackerError unpacker_iter_point(Unpacker *thiz,
                                  uint8_t *point_mem,
                                  uintptr_t point_mem_sz,
                                  PointDesc *out_point);

UnpackerError unpacker_delete(Unpacker **this_ptr);

void unpacker_print_state(const Unpacker *thiz);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
