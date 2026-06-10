#ifndef ASN1_DUMMY_H
#define ASN1_DUMMY_H

#include <stdint.h>
#include <stddef.h>

// A dummy representation of a Basic Safety Message (BSM)
typedef struct {
    uint32_t vehicle_id;
    int32_t latitude;
    int32_t longitude;
    uint16_t speed;
    int16_t heading;
} BSMData;

// Encode a BSMData structure into a dummy ASN.1 byte buffer
// Returns 0 on success, -1 on failure
int asn1_encode(const BSMData* in_data, uint8_t* out_buffer, size_t* out_len);

// Decode a dummy ASN.1 byte buffer into a BSMData structure
// Returns 0 on success, -1 on failure
int asn1_decode(const uint8_t* in_buffer, size_t in_len, BSMData* out_data);

#endif // ASN1_DUMMY_H
