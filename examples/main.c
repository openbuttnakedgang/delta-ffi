
#ifdef WIN32 
   #define PACKED 
#elif
    #define PACKED __attribute__((packed))
#endif

#include <my_header.h>

#include <stdio.h>

int main() {
    printf("VERSION: %s\n", delta_lib_version());

    Unpacker* unpacker = unpacker_new();
    //unpacker_unpack_block(unpacker);
    //unpacker_print_state(unpacker);
    unpacker_delete(&unpacker);

    printf("Unpacker pointer disposed: %p\n", unpacker);
}