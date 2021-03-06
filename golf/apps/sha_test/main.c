#include <firestorm.h>
#include <gpio.h>
#include <string.h>
#include <stdio.h>

#include "digest.h"

// TODO: Kernel bug? can't `allow` const data
static /*const*/ char input_data[] = "Hello World!\n";

static const uint8_t sha1_sum[160 / 8] = {
  0xA0, 0xB6, 0x59, 0x39, 0x67, 0x0B, 0xC2, 0xC0,
  0x10, 0xF4, 0xD5, 0xD6, 0xA0, 0xB3, 0xE4, 0xE4,
  0x59, 0x0F, 0xB9, 0x2B
};

static const uint8_t sha256_sum[256 / 8] = {
  0x03, 0xBA, 0x20, 0x4E, 0x50, 0xD1, 0x26, 0xE4,
  0x67, 0x4C, 0x00, 0x5E, 0x04, 0xD8, 0x2E, 0x84,
  0xC2, 0x13, 0x66, 0x78, 0x0A, 0xF1, 0xF4, 0x3B,
  0xD5, 0x4A, 0x37, 0x81, 0x6B, 0x6A, 0xB3, 0x40
};

static uint8_t hash_output[256 / 8];

/* Delay for for the given microseconds (approximately).
 *
 * For a 16 MHz CPU, 1us == 16 instructions (assuming each instruction takes
 * one cycle). */
static void busy_delay_us(int duration)
{
        // The inner loop instructions are: 14 NOPs + 1 SUBS/ADDS + 1 CMP
        while (duration-- != 0) {
                __asm volatile (
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                        "nop\n"
                );
        }
}

/* Delay for for the given milliseconds (approximately).
 *
 * Note that this is not precise as there are 2 extra instructions on the inner
 * loop. Therefore, there is 1us added every 8 iterations. */
static void busy_delay_ms(int duration)
{
        while (duration-- != 0) {
                busy_delay_us(1000);
        }
}

static void print_buffer(const uint8_t* buf, size_t len) {
  for (size_t i = 0; i < len; ++i) {
    printf("%02x", buf[i]);
  }
}

int main(void) {
  gpio_enable_output(LED_0);
  gpio_set(LED_0);

  int mode = DIGEST_MODE_SHA1;

  printf("Hashing \"%s\"\n", input_data);

  memset(hash_output, 0, sizeof(hash_output));
  int ret = tock_digest_hash_easy(input_data, strlen(input_data),
                                  hash_output, sizeof(hash_output), mode);
  if (ret < 0) goto error;

  size_t hash_size = mode == DIGEST_MODE_SHA1 ? (160 / 8) : (256 / 8);
  const uint8_t* reference_hash = mode == DIGEST_MODE_SHA1 ? sha1_sum  : sha256_sum;

  printf("Result:   ");
  print_buffer(hash_output, hash_size);
  printf("\n");
  printf("Expected: ");
  print_buffer(reference_hash, hash_size);
  printf("\n");

  int result = memcmp(hash_output, reference_hash, hash_size);
  gpio_clear(LED_0);

  while (1) {
    if (result != 0) {
      gpio_toggle(LED_0);
      busy_delay_ms(250);
    }
  }
  return 0;

error:
  printf("Error: %d\n", ret);
  gpio_clear(LED_0);
  while (1) {
    gpio_toggle(LED_0);
    busy_delay_ms(1000);
  }
  return 1;
}

