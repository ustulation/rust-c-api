#include <stdio.h>

struct Inner {
    unsigned char e;
};

struct Abc {
    struct Inner* ptr;
    size_t len;
    unsigned char arr[5];
};

extern void foo(void(*)(struct Abc*));
extern void get_arr(void(*)(unsigned char*));
extern void print_arr(unsigned char*);

void c_cb(struct Abc *abc) {
    printf("In c_cb: len: %zu ;; (len - 1)th element: %u\n", (*abc).len, (*abc).ptr[(*abc).len - 1].e);
    for(int i = 0; i < 5; ++i) printf("%u, ", (*abc).arr[i]);
    printf("\n");
}

void arr_cb(unsigned char(*a)[32]) {
    printf("In C cb - array:\n[");
    for(int i = 0; i < 32; ++i) printf("%u, ", a[0][i]);
    printf("]\nIn C cb - calling rust print..\n");
    print_arr(a);
}

int main() {
    // printf("Calling foo in rust from C\n");
    // foo(c_cb);
    // printf("Exiting main...\n");

    printf("Calling get_arr in rust from C\n");
    get_arr(arr_cb);

    printf("Calling get_arr_wrong in rust from C\n");
    get_arr_wrong(arr_cb);
    printf("Exiting main...\n");
}
