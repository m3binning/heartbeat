.POSIX:
.PHONY: all clean

CC = cc
CFLAGS = -I libprom-dev-0.1.3-Linux/include -I include -std=c11
LDFLAGS = -L libprom-dev-0.1.3-Linux/lib -lprom -lpromhttp -lmicrohttpd

BIN = heartbeat
OBJ = main.o heart.o fuel.o
SRC = main.c heart.c fuel.c

all: $(BIN)

$(BIN): $(OBJ)
	$(CC) -o $@ $^ $(LDFLAGS)

main.o: main.c
heart.o: heart.c
fuel.o: fuel.c

clean:
	$(RM) $(BIN) $(OBJ)
