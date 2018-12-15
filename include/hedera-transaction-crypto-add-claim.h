#pragma once

#include "hedera-transaction.h"
#include "hedera-claim.h"
#include "hedera-id.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__crypto_add_claim__new(
    HederaClient*,
    HederaAccountId account_id,
    HederaClaim claim
);

#ifdef __cplusplus
}
#endif
