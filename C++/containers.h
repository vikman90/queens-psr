/*******************************************************************************
 * N queens problem (C++ version)
 * Support container classes
 *
 * Copyleft 2013 Vikman - All rights revoked.
 * vikman90.blogspot.com - vmfdez90@gmail.com
 * March 25, 2016
 *
 ******************************************************************************/

#ifndef CONTAINERS_H
#define CONTAINERS_H

struct Pair
{
    int index;
    int value;
};

class Set
{
public:

    // Constructor
    explicit Set(int size);

    // Copy constructor
    Set(const Set &other);

    // Destructor
    ~Set();

    // Insert a value
    void insert(int value);

    // Erase a value
    int erase(int value);

    // Clear set
    void clear();

    // Select all available indices from a row, return number of values got
    int selectValues(int *values) const;

    // Get a value (the first one, or -1)
    int value() const;

    // Get number of set cells
    inline int count() const
    {
        return ncount;
    }

    // Assignation operator
    Set& operator=(const Set &other);

private:
    char *array;    // Binary array
    int nsize;      // Actual size of array
    int ncount;     // Number of set cells
};

#endif
