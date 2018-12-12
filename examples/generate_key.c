#include <stdlib.h>
#include <stdio.h>

#include "hedera.h"

int main() {
    HederaSecretKey secret = hedera_secret_key_generate();
    HederaPublicKey public = hedera_public_key_from_secret_key(&secret);

    char *secret_s = hedera_secret_key_to_str(&secret);
    char *public_s = hedera_public_key_to_str(&public);

    printf("secret = %s\n", secret_s);
    printf("public = %s\n", public_s);

    free(secret_s);
    free(public_s);

    return 0;
}
