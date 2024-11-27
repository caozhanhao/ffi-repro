#include <inttypes.h>
#include <stdio.h>

typedef enum {	A1, B1, C1, D1 = 0x1122334455 } TEST1;

typedef enum {	A2, B2, C2, D2 = 65537 } TEST2;

extern TEST1 func_from_rust1();

extern TEST2 func_from_rust2();

extern uint64_t func_from_rust3();


int main(void) {
	printf("TEST size: %d   %d   %d\n", sizeof(TEST1), sizeof(TEST2), sizeof(uint64_t));
	printf("func_from_rust1: %d\n", func_from_rust1() == C1);
	printf("func_from_rust2: %d\n", func_from_rust2() == C2);
	printf("func_from_rust3: %d\n", func_from_rust3() == 0xfffffffff);
	return 0;
}
