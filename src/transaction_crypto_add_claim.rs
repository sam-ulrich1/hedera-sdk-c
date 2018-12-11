use hedera::transaction::TransactionCryptoAddClaim;
use hedera::{ AccountId, Claim };

def_tx_new!(TransactionCryptoAddClaim: hedera_transaction__crypto_add_claim__new(AccountId, Claim));
