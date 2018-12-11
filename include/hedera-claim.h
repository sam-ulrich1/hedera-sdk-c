#pragma once

#include <stdint.h>
#include "hedera-id.h"
#include "hedera-crypto.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HederaClaim HederaClaim;

extern HederaClaim hedera_claim_new(HederaAccountId account, const uint8_t* hash,);

extern void hedera_claim_add_key(HederaClaim*, HederaPublicKey key);

#ifdef __cplusplus
}
#endif
