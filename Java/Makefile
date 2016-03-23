################################################################################
# N queens problem (Java version)
# Makefile
#
# Copyleft 2016 Vikman - All rights revoked.
# vikman90.blogspot.com - vmfdez90@gmail.com
# March 22, 2016
#
# Usage: make [all|clean]
################################################################################

TARGET	= queens.jar
JC		= javac
JAR		= jar

.PHONY: all clean

all: $(TARGET)

clean:
	$(RM) $(TARGET) queens/*.class

$(TARGET): queens/Chess.class queens/Main.class
	$(JAR) cfe $@ queens/Main $^

queens/%.class: queens/%.java
	$(JC) $<
