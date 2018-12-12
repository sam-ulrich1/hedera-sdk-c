#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int64_t seconds;
    uint32_t nanos;
} HederaTimestamp;

extern HederaTimestamp hedera_timestamp_now();

#ifdef __cplusplus
}
#endif
