#pragma once

#include "hedera-transaction.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__file_create__new(HederaClient*);

extern void hedera_transaction__file_create__set_key(HederaTransaction*, HederaPublicKey key);

extern void hedera_transaction__file_create__set_contents(HederaTransaction*, const uint8_t* contents);

#ifdef __cplusplus
}
#endif