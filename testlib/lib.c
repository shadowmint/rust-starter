#include <stdio.h>
#include <stdint.h>

typedef void (*rust_callback)(int32_t);
rust_callback _cb = NULL;

void bind(rust_callback cb) { 
  _cb = cb;
}

void invoke() { 
  if (_cb != NULL) {
    printf("Invoking callback\n");
    (*_cb) (111);
  }
  else {
    printf("No callback found\n");
  }
}
