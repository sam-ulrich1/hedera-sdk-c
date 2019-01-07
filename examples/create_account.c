#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

#include "hedera.h"

void print_hedera_error() {
    char* err = hedera_last_error();
    fprintf(stderr, "error: %s\n", err);
    free(err);
}

int main() {
    // Generate a key pair for the new account to use

    char* mnemonic;
    HederaSecretKey secret = hedera_secret_key_generate("", &mnemonic);
    HederaPublicKey public = hedera_public_key_from_secret_key(&secret);

    char *secret_s = hedera_secret_key_to_str(&secret);
    char *public_s = hedera_public_key_to_str(&public);

    printf("secret = %s\n", secret_s);
    printf("public = %s\n", public_s);

    free(secret_s);
    free(public_s);
    free(mnemonic);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee

    char* operator_s = getenv("OPERATOR");
    if (!operator_s) {
        fprintf(stderr, "error: OPERATOR env variable must be set\n");
        return EXIT_FAILURE;
    }

    HederaAccountId operator;
    if (hedera_account_id_from_str(operator_s, &operator) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        return EXIT_FAILURE;
    }

    // Operator secret is the secret key for this operator
    // We need this to sign our transaction to create our new account

    char* operator_secret_s = getenv("OPERATOR_SECRET");
    if (!operator_secret_s) {
        fprintf(stderr, "error: OPERATOR_SECRET env variable must be set\n");
        return EXIT_FAILURE;
    }

    HederaSecretKey operator_secret;
    if (hedera_secret_key_from_str(operator_secret_s, &operator_secret) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        return EXIT_FAILURE;
    }

    // Node is the ID for the node that we are accessing
    // Currently all testnets have a node ID of `0:0:3`
    // This should be provided via the Hedera Portal for mainnet

    HederaAccountId node = { 0, 0, 3 };

    // Create the Hedera client
    // A client connects to a single Hedera node
    // The address for your specific testnet is available on the Hedera Portal

    HederaClient* client;
    if (hedera_client_new("testnet.hedera.com:50003", &client) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        return EXIT_FAILURE;
    }

    // Create our account
    // The transaction works like a builder where we initialize and set various options before execution

    HederaTransaction* tx = hedera_transaction__crypto_create__new(client);
    hedera_transaction_set_operator(tx, operator);
    hedera_transaction_set_node(tx, node);
    hedera_transaction_set_memo(tx, "[hedera-sdk-c][example] create_account");
    hedera_transaction__crypto_create__set_key(tx, public);
    hedera_transaction__crypto_create__set_initial_balance(tx, 15);
    hedera_transaction_sign(tx, &operator_secret);

    // When a transaction is executed all we receive back is the ID of
    // the transaction.

    HederaTransactionId id;
    if (hedera_transaction_execute(tx, &id) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        hedera_client_close(client);
        return EXIT_FAILURE;
    }

    // If there was an immediate error we are informed of the pre-check failure.
    // Otherwise, we can be reasonably sure it succeeded at this point.

    char* id_s = hedera_transaction_id_to_str(&id);
    printf("created account; transaction = %s\n", id_s);
    free(id_s);

    // Wait 5 seconds ...
    // We wait to make sure the server has enough time to process before we request the receipt
    // For a real application you would loop this as you may receive UNKNOWN for receipt status
    // which would tell you to wait awhile and try again
    sleep(5);

    // Grab the transaction receipt
    HederaTransactionReceipt receipt;
    HederaQuery *query = hedera_query__transaction_get_receipt__new(client, id);
    if (hedera_query__transaction_get_receipt__get(query, &receipt) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        hedera_client_close(client);
        return EXIT_FAILURE;
    }

    // Print our new account ID
    printf("account = %lli\n", receipt.account_id->account);

    // Drop the connection to the Hedera node
    hedera_client_close(client);

    return EXIT_SUCCESS;
}
