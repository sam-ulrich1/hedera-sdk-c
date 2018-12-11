#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int64_t seconds;
    int32_t nanos;
} HederaDuration;

#ifdef __cplusplus
}
#endif
