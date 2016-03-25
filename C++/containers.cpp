/*******************************************************************************
 * N queens problem (C++ version)
 * Support container classes
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 25, 2016
 *
 ******************************************************************************/

#include "containers.h"
#include <cstring>

//------------------------------------------------------------------------------
// Constructor

Set::Set(int size) : array(new char[size]), nsize(size), ncount(size)
{
    memset(array, 1, size);
}

//------------------------------------------------------------------------------
// Destructor

Set::~Set()
{
    delete [] array;
}

//------------------------------------------------------------------------------
// Copy constructor

Set::Set(const Set &other) : array(new char[other.nsize]), nsize(other.nsize), ncount(other.ncount)
{
    memcpy(array, other.array, nsize);
}

//------------------------------------------------------------------------------
// Insert a value

void Set::insert(int value)
{
    if (!array[value]) {
        array[value] = 1;
        ncount++;
    }
}

//------------------------------------------------------------------------------
// Erase a value

int Set::erase(int value)
{
    if (value >= 0 && value < nsize && array[value]) {
        array[value] = 0;
        ncount--;
        return 1;
    } else
        return 0;
}

//------------------------------------------------------------------------------
// Clear set

void Set::clear()
{
    memset(array, 0, nsize);
    ncount = 0;
}

//------------------------------------------------------------------------------
// Select all available indices from a row, return number of values got

int Set::selectValues(int *values) const
{
    int nvalues = 0;

    for (int i = 0; i < nsize; i++) {
        if (array[i])
            values[nvalues++] = i;
    }

    return nvalues;
}

//------------------------------------------------------------------------------
// Get a value (the first one, or -1)

int Set::value() const
{
    for (int i = 0; i < nsize; i++)
        if (array[i])
            return i;

    return -1;
}

// Assignation operator
Set& Set::operator=(const Set &other)
{
    if (this != &other) {
        if (nsize != other.nsize) {
            delete [] array;
            array = new char[other.nsize];
            nsize = other.nsize;
        }

        memcpy(array, other.array, nsize);
        ncount = other.ncount;
    }

    return *this;
}