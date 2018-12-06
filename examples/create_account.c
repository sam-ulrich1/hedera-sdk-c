#include <stdlib.h>
#include <stdio.h>

#include "hedera.h"

void print_hedera_error() {
    char* err = hedera_last_error();
    fprintf(stderr, "error: %s\n", err);
    free(err);
}

int main() {
    // Generate a key pair for the new account to use

    HederaSecretKey secret = hedera_secret_key_generate();
    HederaPublicKey public = hedera_public_key_from_secret_key(&secret);

    char *secret_s = hedera_secret_key_to_str(&secret);
    char *public_s = hedera_public_key_to_str(&public);

    printf("secret = %s\n", secret_s);
    printf("public = %s\n", public_s);

    free(secret_s);
    free(public_s);

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
    if (hedera_client_new("testnet.hedera.com:50001", &client) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        return EXIT_FAILURE;
    }

    // todo: create the account

    // Drop the connection to the Hedera node
    hedera_client_close(client);

    return EXIT_SUCCESS;
}
