CC = gcc
CFLAGS = -Wall -Werror -fPIC -shared

all: libasn1.so

libasn1.so: asn1_dummy.c asn1_dummy.h
	$(CC) $(CFLAGS) asn1_dummy.c -o libasn1.so

clean:
	rm -f libasn1.so
