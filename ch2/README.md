# Chapter 2: Representing and Mainpulating Information

Binary bits, as opposed ot 10-based decimal system, is used in computers as they
are easy to represent and densly pack physically. These bits can be interpreted
in a number of ways, i.e. as numbers or characters. The most common way to
represent numbers is either with *two's-complement* for integer values or
*floating-point* for decimals. Integer values are represented exactly but on
a smaller range where-as decimals are approximated but have a much larger range.

## 2.1 Information Storage

Computers use blocks of 8-bits (*bytes*) to address memory. Memory is organized
as a large array of bytes known as a *virtual address* space. In reality the
memory is split between a number of different places, but computer this
abstraction as a single large array.
