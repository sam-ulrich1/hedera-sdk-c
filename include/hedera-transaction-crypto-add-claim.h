#pragma once

#include "hedera-transaction.h"
#include "hedera-id.h"
#include "hedera-claim.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__add_claim__new(HederaClient*, HederaAccountId id, HederaClaim claim);

#ifdef __cplusplus
}
#endif
