#include "asn1_dummy.h"
#include <string.h>

// Dummy implementation of ASN.1 encoding.
// In a real scenario, this would use a library like asn1c to generate
// proper DER or PER encoded bytes. Here, we just do a simple memcpy for demo purposes.
int asn1_encode(const BSMData* in_data, uint8_t* out_buffer, size_t* out_len) {
    if (!in_data || !out_buffer || !out_len) {
        return -1;
    }

    // A real encoder would serialize the fields properly according to ASN.1 PER/DER
    // For this dummy, we just copy the struct memory.
    size_t size = sizeof(BSMData);
    memcpy(out_buffer, in_data, size);
    *out_len = size;

    return 0;
}

// Dummy implementation of ASN.1 decoding.
int asn1_decode(const uint8_t* in_buffer, size_t in_len, BSMData* out_data) {
    if (!in_buffer || !out_data) {
        return -1;
    }

    if (in_len != sizeof(BSMData)) {
        // Invalid size for our dummy representation
        return -1;
    }

    memcpy(out_data, in_buffer, in_len);
    return 0;
}
