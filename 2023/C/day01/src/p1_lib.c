#include <string.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

#include "p1_lib.h"

int get_first(char input[]) {
	long input_len = strlen(input);
	for (int i = 0; i < input_len; i++) {
		// Loops forwards through input
		if (isdigit(input[i])) {
			return (input[i] - '0') * 10;
			// Multiply by ten because the first number is in the 10s column
		}
	}

	fprintf(stderr, "Error finding first number in %s", input);
	exit(1);
}

int get_last(char input[]) {
	long input_len = strlen(input);
	for (int i = input_len - 1; i >= 0; i--) {
		// Loops backwards through input
		if (isdigit(input[i])) {
			return input[i] - '0';
			// Converts input[i] to an int
		}
	}

	fprintf(stderr, "Error finding last number in %s", input);
	exit(1);
}

int process(char input[]) {
	int first_num = get_first(input);
	int last_num = get_last(input);
	int num = first_num + last_num;

	return num;
}
