#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct AutoTyper {
  char *file;
  uint8_t start_delay;
  double wpm;
  uintptr_t next_stage;
} AutoTyper;

struct AutoTyper create(void);

void set_file(struct AutoTyper *self, char *file);

void set_start_delay(struct AutoTyper *self, uint8_t start_delay);

void set_wpm(struct AutoTyper *self, double wpm);

void configure(struct AutoTyper *self);

bool has_next(struct AutoTyper *self);

void next(struct AutoTyper *self);

void print(const struct AutoTyper *self);
