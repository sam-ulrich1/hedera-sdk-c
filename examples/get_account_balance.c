#include <stdlib.h>
#include <stdio.h>

#include "hedera.h"

void print_hedera_error() {
    char* err = hedera_last_error();
    fprintf(stderr, "error: %s\n", err);
    free(err);
}

int get_operator_secret(void* user_data, HederaSecretKey* out) {
    char* operator_secret_s = getenv("OPERATOR_SECRET");
    if (!operator_secret_s) {
        fprintf(stderr, "error: OPERATOR_SECRET env variable must be set\n");
        return EXIT_FAILURE;
    }

    if (hedera_secret_key_from_str(operator_secret_s, out) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();

        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}

int main() {
    HederaAccountId target = { 0, 0, 2 };

    HederaClient* client;
    if (hedera_client_new("testnet.hedera.com:50003", &client) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();

        return EXIT_FAILURE;
    }

    HederaAccountId node = { 0, 0, 3 };
    hedera_client_set_node(client, node);

    hedera_client_set_operator(client, target, &get_operator_secret, NULL);

    HederaQuery* query = hedera_query__crypto_get_account_balance__new(client, target);

    uint64_t balance;
    if (hedera_query__crypto_get_account_balance__get(query, &balance) != HEDERA_ERROR_SUCCESS) {
        print_hedera_error();
        hedera_client_close(client);

        return EXIT_FAILURE;
    }

    // note: the query object is freed when used by `get` or `cost`

    printf("balance = %lld tinybars\n", balance);
    printf("balance = %lf hbars\n", ((double)(balance)) / 100000000.0);

    hedera_client_close(client);

    return EXIT_SUCCESS;
}
