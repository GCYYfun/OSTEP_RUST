
# Overview

This program allows you to see how address translations are performed in a
system with base and bounds registers. As before, there are two steps to
running the program to test out your understanding of base and bounds. First,
run without the -c flag to generate a set of translations and see if you can
correctly perform the address translations yourself. Then, when done, run with
the -c flag to check your answers.

In this homework, we will assume a slightly different address space than our
canonical one with a heap and stack at opposite ends of the space. Rather, we
will assume that the address space has a code section, then a fixed-sized
(small) stack, and a heap that grows downward right after, looking something
like you see in the Figure below. In this configuration, there is only one
direction of growth, towards higher regions of the address space.

```sh
  -------------- 0KB
  |    Code    |
  -------------- 2KB
  |   Stack    |
  -------------- 4KB
  |    Heap    |
  |     |      |
  |     v      |
  -------------- 7KB
  |   (free)   |
  |     ...    |
```

In the figure, the bounds register would be set to 7~KB, as that represents
the end of the address space. References to any address within the bounds
would be considered legal; references above this value are out of bounds and
thus the hardware would raise an exception.

To run with the default flags, type relocation.py at the command line. The
result should be something like this:

```sh
prompt> cargo run relocation
...
Base-and-Bounds register information:

  Base   :  0x19c2   (decimal 6594 )
  Limit  :   396 

Virtual Address Trace
  VA  0 : 0x3f0 (decimal : 1008 )--> PA or segmentation violation?
  VA  1 : 0x52 (decimal : 82 )--> PA or segmentation violation?
  VA  2 : 0xd8 (decimal : 216 )--> PA or segmentation violation?
  VA  3 : 0x262 (decimal : 610 )--> PA or segmentation violation?
  VA  4 : 0x18c (decimal : 396 )--> PA or segmentation violation?
```

For each virtual address, either write down the physical address it 
translates to OR write down that it is an out-of-bounds address 
(a segmentation violation). For this problem, you should assume a 
simple virtual address space of a given size.

As you can see, the homework simply generates randomized virtual
addresses. For each, you should determine whether it is in bounds, and if so,
determine to which physical address it translates. Running with -c (the
"compute this for me" flag) gives us the results of these translations, i.e.,
whether they are valid or not, and if valid, the resulting physical
addresses. For convenience, all numbers are given both in hex and decimal.

```sh
prompt> cargo run relocation -c
...
Virtual Address Trace
  VA  0 : 0x3f0 (decimal : 1008 ) --> SEGMENTATION VIOLATION
  VA  1 : 0x52 (decimal : 82 ) --> VALID: 0x1a14 (decimal: 6676)
  VA  2 : 0xd8 (decimal : 216 ) --> VALID: 0x1a9a (decimal: 6810)
  VA  3 : 0x262 (decimal : 610 ) --> SEGMENTATION VIOLATION
  VA  4 : 0x18c (decimal : 396 ) --> SEGMENTATION VIOLATION
```

With a base address of 6594 (decimal), address 1008 is out  bounds (i.e., it
is more than the limit register of 396) and thus translates to 1008 added to
6594 or 7602. A few of the addresses shown above are within (82,
216), as they are inferior to the bounds. Pretty simple, no? Indeed, that is
one of the beauties of base and bounds: it's so darn simple!

There are a few flags you can use to control what's going on better:

```sh
prompt> cargo run relocation -h
Usage: relocation [options]

Options:
  -h, --help            show this help message and exit
  -s SEED, --seed=SEED  the random seed
  -a ASIZE, --asize=ASIZE address space size (e.g., 16, 64k, 32m)
  -p PSIZE, --physmem=PSIZE physical memory size (e.g., 16, 64k)
  -n NUM, --addresses=NUM # of virtual addresses to generate
  -b BASE, --b=BASE     value of base register
  -l LIMIT, --l=LIMIT   value of limit register
  -c, --compute         compute answers for me
```

In particular, you can control the virtual address-space size (-a), the size
of physical memory (-p), the number of virtual addresses to generate (-n), and
the values of the base and bounds registers for this process (-b and -l,
respectively).

