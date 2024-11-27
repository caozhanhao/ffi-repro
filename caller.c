#include <inttypes.h>
#include <stdio.h>

typedef enum {	A1, B1, C1 = 0x1122334455 } TEST1;

typedef enum {	A2, B2, C2 = 65537 } TEST2;

extern TEST1 func_from_rust1(void (*)(void*), void*, unsigned int*, unsigned short);

extern TEST2 func_from_rust2(void (*)(void*), void*, unsigned int*, unsigned short);

extern uint64_t func_from_rust3(void (*)(void*), void*, unsigned int*, unsigned short);


int main(void) {
	printf("TEST size: %d   %d   %d\n", sizeof(TEST1), sizeof(TEST2), sizeof(uint64_t));
	printf("func_from_rust1: %d\n", func_from_rust1(0, 0, 0, 0) == A1);
	printf("func_from_rust2: %d\n", func_from_rust2(0, 0, 0, 0) == A2);
	printf("func_from_rust3: %d\n", func_from_rust3(0, 0, 0, 0) == 0);
	return 0;
}
