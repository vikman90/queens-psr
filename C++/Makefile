################################################################################
# N queens problem (C++ version)
# Makefile
#
# Copyleft 2016 Vikman - All rights revoked.
# vikman90.blogspot.com - vmfdez90@gmail.com
# March 22, 2016
#
# Usage: make [all|clean]
################################################################################

TARGET		= queens
CXXFLAGS	= -O2 -pipe -Wall -Wextra -std=c++11

.PHONY: all clean

all: $(TARGET)

clean:
	$(RM) queens

$(TARGET): main.cpp chess.cpp
	$(CXX) $(CXXFLAGS) -o $@ $^

main.cpp: chess.h
chess.cpp: chess.h
