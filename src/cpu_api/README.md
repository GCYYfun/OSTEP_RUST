
# Overview: `fork.py`

The simulator `fork` is a simple tool to show what a process tree
looks like when processes are created and destroyed.

To run it, just:
```sh
prompt> cargo run fork
```

What you'll see then is a list of actions, such as whether a process
calls `fork` to create another process, or whether a process calls
`exit` to stop running.

Each process that is running can have multiple children (or
none). Every process, except the initial process (which we call `A`
here for simplicity), has a single parent. Thus, all processes are
related in a tree, rooted at the initial process. We will call this
tree the `Process Tree` and understanding what it looks like as
processes are created and destroyed is the point of this simple
homework. 

# Simple Example

Here is a simple example:
```sh
prompt>  cargo run fork -s 4

                   Process Tree:
                                   A
Action:A forks B
Process Tree?
Action:B EXITS
Process Tree?
Action:A forks C
Process Tree?
Action:C EXITS
Process Tree?
Action:A forks D
Process Tree?
```

From the output, you can see two things. First, on the right, is the
initial state of the system. As you can see, it contains one process,
`A`. Operating systems often create one or a few initial processes to
get things going; on Unix, for example, the initial process is called
`init` which spawns other processes as the system runs.

Second, on the left, you can see a series of `Action` listings, in
which various actions take place, and then a question is posed about
the state of the process tree is at that point.

To solve, and show all outputs, use the `-c` flag, as follows:
```sh
prompt> ./fork.py -s 4 -c

                  Process Tree:
                                   A
Action:A forks B
                                   A
                                   └── B
Action:B EXITS
                                   A
Action:A forks C
                                   A
                                   └── C
Action:C EXITS
                                   A
Action:A forks D
                                   A
                                   └── D
prompt>
```

As you can see, the expected tree that results (shown left-to-right)
from a particular operation is shown now. After the first action, `A
forks B`, you see a very simple tree, with `A` shown as `B`'s
parent. After a few more forks, a call to `exit` is made by `B`, which
reduces the tree. Finally, `D` is created, and the final tree, with
`A` as parent of `D`,as the final state.

In a simplified mode, you can just test yourself by trying to write
down the final process tree, using the `-F` flag:

```sh
prompt> cargo run fork -s 4 -F
                   Process Tree:
                                   A

Action:A forks B
Action:B EXITS
Action:A forks C
Action:C EXITS
Action:A forks D

             Final Process Tree?
```

Once again, you can use the `-c` flag to compute the answer and see if
you were right (in this case, you should be, because it's the same
problem!)

# Other Options

A number of other options exist with the `fork` simulator.

You can flip the question around with the `-t` flag, which allows you
to view process tree states and then guess what action must have taken
place.

You can use different random seeds (`-s` flag) or just don't specify
one to get different randomly generated sequences.

You can change what percent of actions are forks (vs exits) with
the `-f` flag.

You can specify specific fork and exit sequences with the `-A`
flag. For example, to have `a` fork `b`, `b` then fork `c`; `c`
exit, and finally, `a` fork `d`, just type (we show `-c` here to solve
the problem, too): 

```sh
prompt> cargo run fork -A A+B,B+C,C-,A+D -c

                   Process Tree:
                                   A

Action:A forks B
                                   A
                                   └── B
Action:B forks C
                                   A
                                   └── B
                                      └── C
Action:C EXITS
                                   A
                                   └── B
Action:A forks D
                                   A
                                   ├── B
                                   └── D
```

You can only show the final output (and see if you can guess all the
intermediates to get there) with the `-F` flag.

Finally, you can change the printing style of the tree with the `-P`
flag. 
