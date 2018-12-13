#pragma  once

#include "hedera-id.h"
#include "hedera-error.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    // response codes for pre check validation
    RESPONSE_OK = 0; // the transaction passed the precheck
    RESPONSE_INVALID_TRANSACTION = 1; // For any error not handled by specific error codes listed below.
    RESPONSE_PAYER_ACCOUNT_NOT_FOUND  = 2; //Payer account does not exist.
    RESPONSE_INVALID_NODE_ACCOUNT=3; //Node Account provided does not match the node account of the node the transaction was submitted to.
    RESPONSE_TRANSACTION_EXPIRED = 4; // Pre-Check TransactionValidStart + transactionValidDuration is less than current consensus time.
    RESPONSE_INVALID_TRANSACTION_START = 5;// Transaction start time is greater than current consensus time
    RESPONSE_INVALID_TRANSACTION_DURATION = 6;//valid transaction duration is a positive non zero number that does not exceed 120 seconds
    RESPONSE_INVALID_SIGNATURE = 7;//the transaction signature is not valid
    RESPONSE_MEMO_TOO_LONG = ;//Transaction memo size exceeded 100 bytes
    RESPONSE_INSUFFICIENT_TX_FEE  = 9; // the transaction fee is insufficient for this type of transaction
    RESPONSE_INSUFFICIENT_PAYER_BALANCE  = 10; // the payer account has insufficient cryptocurrency to pay the transaction fee
    RESPONSE_DUPLICATE_TRANSACTION = 11; // this transaction ID is a duplicate of one that was submitted to this node or reached consensus in the last 180 seconds (receipt period)
    RESPONSE_BUSY = 12; //If API is throttled out
    RESPONSE_NOT_SUPPORTED = 13; //not supported API

    //response codes used in queries
    RESPONSE_INVALID_FILE_ID = 14;//the file id is invalid or does not exist
    RESPONSE_INVALID_ACCOUNT_ID = 15;//the account id is invalid or does not exist
    RESPONSE_INVALID_CONTRACT_ID = 16;//the contract id is invalid or does ont exist
    RESPONSE_INVALID_TRANSACTION_ID =17;//transaction id is not valid
    RESPONSE_RECEIPT_NOT_FOUND = 18;//receipt for given transaction id does not exist
    RESPONSE_RECORD_NOT_FOUND = 19;//record for given transaction id does not exist
    RESPONSE_INVALID_SOLIDITY_ID = 20;//the solidity id is invalid or entity with this solidity id does not exist

    // response code for Transaction receipt
    RESPONSE_UNKNOWN = 21; // hasn't yet reached consensus, or has already expired
    RESPONSE_SUCCESS = 22; // the transaction succeeded
    RESPONSE_FAIL_INVALID = 23; // the transaction failed because it is invalid
    RESPONSE_FAIL_FEE = 24; // the transaction fee was insufficient
    RESPONSE_FAIL_BALANCE = 25; // the paying account had insufficient cryptocurrency

    //Crypto Response codes
    RESPONSE_KEY_REQUIRED = 26; //Key not provided in the transaction body
    RESPONSE_BAD_ENCODING = 27; //Unsupported algorithm/encoding used for keys in the transaction
    RESPONSE_INSUFFICIENT_ACCOUNT_BALANCE = 28; //When the account balance is not sufficient for the transfer
    RESPONSE_INVALID_SOLIDITY_ADDRESS = 29; //During an update transaction when the system is not able to find the Users Solidity address

    //Smart contract creation or execution
    RESPONSE_INSUFFICIENT_GAS = 30; //Not enough gas was supplied to execute tranasction
    RESPONSE_CONTRACT_SIZE_LIMIT_EXCEEDED =31; //contract byte code size is over the limit
    RESPONSE_LOCAL_CALL_MODIFICATION_EXCEPTION=32;//local execution (query) is requested for a function which changes state
    RESPONSE_CONTRACT_REVERT_EXECUTED=33; //Contract REVERT OPCODE executed
    RESPONSE_CONTRACT_EXECUTION_EXCEPTION=34;//For any contract execution related error not handled by specific error codes listed above.
    RESPONSE_INVALID_RECEIVING_NODE_ACCOUNT = 35; //In Query validation, account with +ve(amount) value should be Receiving node account, the receiver account should be only one account in the list
    RESPONSE_MISSING_QUERY_HEADER = 36; // Header is missing in Query request
    RESPONSE_ACCOUNT_UPDATE_FAILED = 37; // the update of the account failed
    RESPONSE_INVALID_KEY_ENCODING = 38;
    RESPONSE_NULL_SOLIDITY_ADDRESS = 39; // null solidity address
    RESPONSE_CONTRACT_UPDATE_FAILED = 40; // update of the contract failed
    RESPONSE_INVALID_QUERY_HEADER = 41; // the query header is invalid
    RESPONSE_INVALID_FEE_SUBMITTED = 42; // Invalid fee submitted*/
    RESPONSE_INVALID_PAYER_SIGNATURE = 43; //  payer signature is invalid

    RESPONSE_KEY_NOT_PROVIDED = 44;
    RESPONSE_INVALID_EXPIRATION_TIME = 45;
    RESPONSE_NO_WACL_KEY = 46;
    RESPONSE_FILE_CONTENT_EMPTY = 47;
    RESPONSE_INVALID_ACCOUNT_AMOUNTS = 48; // The crypto transfer credit and debit don't equal to 0

    // new response codes to be added
    RESPONSE_EMPTY_TRANSACTION_BODY = 49; // transaction body is empty
    RESPONSE_INVALID_TRANSACTION_BODY = 50; // invalid transaction body
} HederaResponseStatus;

typedef enum HederaResponseStatus


typedef struct HederaClient HederaClient;

/// Establish a connection to a Hedera node.
/// Must be closed with [hedera_client_close].
extern HederaError hedera_client_new(const char* address, HederaClient**);

/// Close and releases resources for a [HederaClient].
extern void hedera_client_close(HederaClient*);

#ifdef __cplusplus
}
#endif
