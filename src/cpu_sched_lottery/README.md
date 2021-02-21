
# Overview

This program, lottery, allows you to see how a lottery scheduler
works. As always, there are two steps to running the program. First, run
without the -c flag: this shows you what problem to solve without
revealing the answers. 

```sh
prompt> cargo run lottery -j 2 -s 0
...
Here is the job list, with the run time of each job: 
  Job 0 ( length = 6, tickets = 65 )
  Job 1 ( length = 7, tickets = 70 )

Here is the set of random numbers you will need (at most):
Random 729870 
Random 465922 
Random 507365 
Random 699143 
Random 560967 
Random 60171 
Random 195347 
Random 879111 
Random 580058 
Random 549531 
Random 414773 
Random 828985 
Random 823539 
```

When you run the simulator in this manner, it first assigns you some random
jobs (here of lengths 6, and 5), each with some number of tickets (here 65 and
70, respectively). The simulator also gives you a list of random numbers,
which you will need to determine what the lottery scheduler will do. The
random numbers are chosen to be between 0 and a large number; thus, you'll
have to use the modulo operator to compute the lottery winner (i.e., winner
should equal this random number modulo the total number of tickets in the
system). 

Running with -c shows exactly what you are supposed to calculate:

```sh
prompt> cargo run lottery -j 2 -s 0 -c
...
** Solutions **
Random 729870 -> Winning ticket 60 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 6 tix: 65 
  job : 1 timeletft: 7 tix: 70 

Random 465922 -> Winning ticket 37 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 5 tix: 65 
  job : 1 timeletft: 7 tix: 70 

Random 507365 -> Winning ticket 35 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 4 tix: 65 
  job : 1 timeletft: 7 tix: 70 

Random 699143 -> Winning ticket 113 (of 135) -> Run 1 
  Jobs:
  job : 0 timeletft: 3 tix: 65 
* job : 1 timeletft: 7 tix: 70 

Random 560967 -> Winning ticket 42 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 3 tix: 65 
  job : 1 timeletft: 6 tix: 70 

Random 60171 -> Winning ticket 96 (of 135) -> Run 1 
  Jobs:
  job : 0 timeletft: 2 tix: 65 
* job : 1 timeletft: 6 tix: 70 

Random 195347 -> Winning ticket 2 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 2 tix: 65 
  job : 1 timeletft: 5 tix: 70 

Random 879111 -> Winning ticket 126 (of 135) -> Run 1 
  Jobs:
  job : 0 timeletft: 1 tix: 65 
* job : 1 timeletft: 5 tix: 70 

Random 580058 -> Winning ticket 98 (of 135) -> Run 1 
  Jobs:
  job : 0 timeletft: 1 tix: 65 
* job : 1 timeletft: 4 tix: 70 

Random 549531 -> Winning ticket 81 (of 135) -> Run 1 
  Jobs:
  job : 0 timeletft: 1 tix: 65 
* job : 1 timeletft: 3 tix: 70 

Random 414773 -> Winning ticket 53 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 1 tix: 65 
  job : 1 timeletft: 2 tix: 70 

--> JOB 0 DONE at time 11
Random 828985 -> Winning ticket 45 (of 70) -> Run 1 
  Jobs:
  job : 0 timeletft: 0 tix: --- 
* job : 1 timeletft: 2 tix: 70 

Random 823539 -> Winning ticket 59 (of 70) -> Run 1 
  Jobs:
  job : 0 timeletft: 0 tix: --- 
* job : 1 timeletft: 1 tix: 70 

--> JOB 1 DONE at time 13
```

As you can see from this trace, what you are supposed to do is use the random
number to figure out which ticket is the winner. Then, given the winning
ticket, figure out which job should run. Repeat this until all of the jobs are
finished running. It's as simple as that -- you are just emulating what the
lottery scheduler does, but by hand!

Just to make this absolutely clear, let's look at the first decision made in
the example above. At this point, we have two jobs (job 0 which has a runtime
of 6 and 65 tickets, and job 1 which has a runtime of 7 and 75 tickets). The
first random number we are given is 729870. As there are 100 tickets in the
system, 729870 % 135 is 60, and thus 60 is our winning ticket.

If ticket 60 is the winner, we simply search through the job list until we
find it. The first entry, for job 0, has 60 tickets (0 through 64), and thus
does win; so we run job 0 for the quantum length (1 in this example). All of this is
shown in the print out as follows:

```sh
Random 729870 -> Winning ticket 60 (of 135) -> Run 0 
  Jobs:
* job : 0 timeletft: 6 tix: 65 
  job : 1 timeletft: 7 tix: 70 
```

As you can see, the first line summarizes what happens, and the second simply
shows the entire job queue, with an * denoting which job was chosen.

The simulator has a few other options, most of which should be
self-explanatory. Most notably, the -l/--jlist flag can be used to specify an
exact set of jobs and their ticket values, instead of always using
randomly-generated job lists.

```sh
prompt> cargo run lottery -h
Usage: lottery.py [options]

Options:
  -h, --help            
      show this help message and exit
  -s SEED, --seed=SEED  
      the random seed
  -j JOBS, --jobs=JOBS  
      number of jobs in the system
  -l JLIST, --jlist=JLIST
      instead of random jobs, provide a comma-separated list
      of run times and ticket values (e.g., 10:100,20:100
      would have two jobs with run-times of 10 and 20, each
      with 100 tickets)
  -m MAXLEN, --maxlen=MAXLEN
      max length of job
  -T MAXTICKET, --maxtick=MAXTICKET
      maximum ticket value, if randomly assigned
  -q QUANTUM, --quantum=QUANTUM
      length of time slice
  -c, --compute
      compute answers for me
```

