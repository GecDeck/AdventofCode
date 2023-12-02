#include <stdio.h>
#include <stdlib.h>
#include "../p1_lib.h"

int main(int argc, char *argv[]) {
	if (argc < 2) {
		fprintf(stderr, "No file given");
		exit(1);
	}

	FILE *file = fopen(argv[1], "r");
	// Opens file supplied by first argument
	
	int sum = 0;
	char line[128] = { 0 };
	// Creates an empty char array with 128 byte capacity for the file to be read into
	while (fgets(line, sizeof(line), file)) {
		// Will keep reading lines from the file into line until file is empty
		int num = process(line);
		// Gets a number from the line
		sum += num;
		// Adds the number to the sum
	}

	printf("%d", sum);
	fclose(file);
}
