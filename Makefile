PREFIX?=/usr/X11R6
CFLAGS?=-Os -pedantic -Wall

all:
	$(CC) $(CFLAGS) -I$(PREFIX)/include *.c -L$(PREFIX)/lib -lX11 -o cwm

clean:
	rm -f cwm
