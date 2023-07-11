#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct Mango {
  int b;
};

struct Mango *new_mango() {
  printf("creating mango\n");
  struct Mango *mango = malloc(sizeof(struct Mango));
  mango->b = 55;
  return mango;
}

void print_mango(struct Mango *mango) {
  if (mango == NULL) {
    printf("Mango is null, can't print\n");
    return;
  }
  printf("mango: %d\n", mango->b);
}

void free_mango(struct Mango *mango) {
  if (mango == NULL) {
    printf("Mango is null, can't free\n");
    return;
  }
  printf("freeing mango\n");
  free(mango);
}

// can't be static
int test_global = 42;

typedef void (*rust_callback)(void*, int32_t);
void* cb_target;
rust_callback cb;

int32_t register_callback(void* callback_target, rust_callback callback) {
  cb_target = callback_target;
  cb = callback;
  return 1;
}

void trigger_callback() {
  cb(cb_target, 7);
}
