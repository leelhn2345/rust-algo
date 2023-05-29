# lines in a box

## question description

write a program to find out the number of possible locations that a line that
stretches across 3 boxes can be placed in a **N** by **M** box.

The line can be horizontal, vertical and diagonal. Objective is to find out
number of possible lines which stretches across the same 3 boxes as shown in
Figure 1.
![Alt text](img/Screenshot%202023-05-29%20203515.png)

In figure 2, the lines displayed are duplicates.
![Alt text](img/Screenshot%202023-05-29%20204118.png)

consider a 4 by 3 box. in total, there are 14 different locations that a line
can be placed in the box. all of the possible locations are displayed in figure 3.
![Alt text](img/Screenshot%202023-05-29%20204257.png)

## constraints

- 0 <= N <= 1000
- 0 <= M <= 1000

N and M are chosen as type `u32` because when the number *1000* is chosen, the
final output is beyond the range of u16. Hence, input is of type of `u16` for
simplicity sake.

## inputs/outputs

the input will be **N** and **M**.

the output will be

```sh
the number of posssible lines: X
```
