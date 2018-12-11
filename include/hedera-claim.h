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
    usize_t hash_len;
    HederaPublicKey* keys;
    usize_t keys_len;
} HederaClaim;

#ifdef __cplusplus
}
#endif
