#include <stdbool.h>
#include <stdio.h>

#include "../src/p1_lib.h"

bool test_process() {
	char test_string[] = "1abc2";
	char test_string2[] = "pqr3stu8vwx";
	char test_string3[] = "a1b2c3d4e5f";
	char test_string4[] = "treb7uchet";

	bool test_pass = false;
	if (process(test_string) == 12 && process(test_string2) == 38 &&
			process(test_string3) == 15 && process(test_string4) == 77) {
				test_pass = true;
			}

	printf("Process Test: %s\n", test_pass ? "Passed" : "Failed");
	if (!test_pass) {
		printf("%d\n%d\n%d\n%d\n",
				process(test_string),
				process(test_string2),
				process(test_string3),
				process(test_string4));
	}
	return test_pass;
}

int main() {
	bool test_proccess_success = test_process();

	return test_proccess_success;
}


