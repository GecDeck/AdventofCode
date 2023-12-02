CC=gcc
CFLAGS=-I src/
DEPS= src/p1_lib.h
OBJ= src/bin/p1.c src/p1_lib.c

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

part1: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
