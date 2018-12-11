#pragma once

#include <stdint.h>
#include "hedera-id.h"
#include "hedera-crypto.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    HederaAccountId account;
    const uint8_t* hash;
    HederaPublicKey keys[];
} HederaClaim;

#ifdef __cplusplus
}
#endif
