#include <stdlib.h>
#include <stdio.h>

#include "hedera.h"

int main() {
    HederaAccountId target = { 0, 0, 2 };

    HederaClient* client;
    if (hedera_client_new("testnet.hedera.com:50001", &client) != HEDERA_ERROR_SUCCESS) {
        char* err = hedera_last_error();
        fprintf(stderr, "error: %s\n", err);
        free(err);

        return EXIT_FAILURE;
    }

    HederaQuery* query = hedera_query__get_account_balance__new(client, target);

    uint64_t balance;
    if (hedera_query__get_account_balance__get(query, &balance) != HEDERA_ERROR_SUCCESS) {
        char* err = hedera_last_error();
        fprintf(stderr, "error: %s\n", err);
        free(err);

        hedera_client_close(client);

        return EXIT_FAILURE;
    }

    // note: the query object is freed when used by `get` or `cost`

    printf("balance = %lld tinybars\n", balance);
    printf("balance = %lf hbars\n", ((double)(balance)) / 100000000.0);

    hedera_client_close(client);

    return EXIT_SUCCESS;
}
