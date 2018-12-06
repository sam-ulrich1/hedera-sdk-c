#include <stdlib.h>
#include <stdio.h>

#include "hedera.h"

int main() {
    HederaSecretKey secret = hedera_secret_key_generate();
    HederaPublicKey public = hedera_public_key_from_secret_key(&secret);

    char *secret_str = hedera_secret_key_to_str(&secret);
    char *public_str = hedera_public_key_to_str(&public);

    printf("secret = %s\n", secret_str);
    printf("public = %s\n", public_str);

    free(secret_str);
    free(public_str);

    return 0;
}
