#include <firestorm.h>
#include <stdint.h>
#include <stdio.h>
#include <tock.h>

int main(void) {
  printf("Hello from the RNG application!\n");

  uint32_t buf;
  int err = allow(5, 0, &buf, 4);
  while (err == 0) {
    printf("Have some random bytes: 0x%lx\n\n", buf);
    delay_ms(1000);
    err = allow(4, 0, &buf, 4);
  }

  return err;
}
