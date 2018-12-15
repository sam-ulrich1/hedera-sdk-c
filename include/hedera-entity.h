#pragma once

 #include <stdint.h>
 #include "hedera-id.h"
 #include "hedera-claim.h"

 #ifdef __cplusplus
 extern "C" {
 #endif

 typedef union {
     HederaAccountId account;
     HederaClaim claim;
     HederaFileId file;
     HederaContractId contract;

} HederaEntity;

 #ifdef __cplusplus
 }
 #endif
