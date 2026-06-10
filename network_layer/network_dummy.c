#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <stdint.h>

#define PORT 8080

// We must use the exact same struct to send valid dummy data
typedef struct {
    uint32_t vehicle_id;
    int32_t latitude;
    int32_t longitude;
    uint16_t speed;
    int16_t heading;
} BSMData;

int main() {
    int sock = 0;
    struct sockaddr_in serv_addr;
    
    printf("[Network Layer] Starting dummy network sender...\n");

    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        printf("\n [Network Layer] Socket creation error \n");
        return -1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);

    // Convert IPv4 and IPv6 addresses from text to binary form
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        printf("\n[Network Layer] Invalid address/ Address not supported \n");
        return -1;
    }

    // Keep trying to connect since the server might not be up yet
    while (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
        printf("[Network Layer] Connection Failed. Retrying in 2 seconds...\n");
        sleep(2);
    }
    
    printf("[Network Layer] Connected to V2X Stack at 127.0.0.1:%d\n", PORT);

    BSMData bsm = {
        .vehicle_id = 1001,
        .latitude = 377749000,   // dummy coords
        .longitude = -1224194000,
        .speed = 50,
        .heading = 90
    };

    while (1) {
        // Send the encoded payload (for our dummy case, we just send the raw struct)
        // In reality, this would be an ASN.1 encoded byte array sent from another node
        bsm.latitude += 100;
        bsm.longitude += 50;
        bsm.speed = (bsm.speed + 1) % 100;
        
        int bytes_sent = send(sock, &bsm, sizeof(BSMData), 0);
        if (bytes_sent < 0) {
            perror("[Network Layer] Send failed");
            break;
        }
        
        printf("[Network Layer] Sent BSM frame for Vehicle ID: %u, Speed: %u\n", bsm.vehicle_id, bsm.speed);
        
        sleep(1);
    }

    close(sock);
    return 0;
}
