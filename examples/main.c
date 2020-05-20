
#ifdef WIN32 
   #define PACKED 
#else
    #define PACKED __attribute__((packed))
#endif

#include "delta_ffi.h"

#include <stdio.h>

int main() {
    printf("VERSION: %s\n", delta_lib_version());

    FnBlockHeader fn_header = delta_blockparser_header();
    BlockHeader* header = fn_header.thys;
    printf("id_inv: %x\n", fn_header.id_inv(header));
    printf("id_fmt: %x\n", fn_header.id_fmt(header));
    printf("rec_num: %x\n", fn_header.rec_num(header));
    printf("blk_num: %x\n", fn_header.blk_num(header));
    printf("time: %llx\n", fn_header.time(header));

    //Unpacker* unpacker = unpacker_new();
    //unpacker_unpack_block(unpacker);
    //unpacker_print_state(unpacker);
    //unpacker_delete(&unpacker);

    //printf("Unpacker pointer disposed: %p\n", unpacker);
}