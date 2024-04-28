CFLAGS = -g -O2 -Wall
LDFLAGS = -lX11

all: main windowlist.o click-actions/raise click-actions/minimize click-actions/close

main: main.c windowlist.o windowlist.h toml-c.h
	gcc $(CFLAGS) $(LDFLAGS) -o main main.c windowlist.o

windowlist.o: windowlist.c
	gcc $(CFLAGS) -c windowlist.c

click-actions/%:
	cd click-actions; cargo build --release --bin `basename $@`; mv target/release/`basename $@` .

clean:
	rm windowlist.o
	rm main
	rm click-actions/raise
	rm click-actions/minimize
	rm click-actions/close
