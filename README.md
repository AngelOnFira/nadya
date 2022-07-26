# Basics

## Belts

In Nadya, you move variables around a factory on conveyor belts, and combine them to create new variables. Belts can either go horizontally with `_` or vertically with `|`. The floor of the factory is represented with periods `.`. A trivial program in Nadya might just be a factory floor.

```
....
....
....
....
```

To make it a bit more interesting, we want to add some belts.

```
....
.__.
..|.
....
```

However, belts on their own have no direction! An output `W` is required for the belts to find their direction.

```
.....
.|...
.__..
..|..
..W..
.....
```

When Nadya reads a `.nya` file, it first looks for the output, and finds all of the paths to it with a breadth-first search. This means that each belt will always take the closest route it can to get to the output. Also, there can only be a single output in a Nadya program. Here's a diagram of each belt replaced with a direction that it moves:

```
.....
.v...
.>v..
..v..
..W..
.....
```

## Variables and Values

In Nadya, variables move around and interact with one another to change their values. Spawners will put new variables onto a belt. While a variable is moving around, you'll only see it in the form `O`, and not by its value. There are two kinds of spawners; file spawners and digit spawners.

### Digit spawners

The digits `0` `1` `2` ... `9` will all spawn variables onto a (single) adjacent belt that contain that value. This program

```
...
.3.
.|.
.W.
...
```

Will output

```
3
3
3
3
...
```

### File spawners

A file spawner can be placed on the factory with `F`. If a file spawner is used at all in a program, Nadya will look for a file `input.txt` and parse each line as an integer. If the file is missing, or any lines can't be parsed as an integer, Nadya will crash. Say an `input.txt` file contains the following:

```
5
4
3
```

And the factory

```
...
.F.
.|.
.W.
...
```

With this factory the following factory, Nadya will produce the output

```
5
4
3
5
4
3
5
4
3
...
```

As we can see, Nadya will loop a file once it reaches the end.

# Operators

Nadya currently has two operators that can be used. Nadya can do addition `+` and multiplication `*`. Here is a program that makes use of operators

```
.......
.1...3.
.|...|.
.__+__.
...|...
.5_*...
...W...
```

This program would output 20. `1` and `3` spawn their values that are summed, and that variable with the new value 4 is multiplied against the value from `5`. This results in a variable that arrives at the output `W` that contains the value 20.


# Gotchas (bugs in disguise)

## Borders

The border of the factory must be surrounded by floor `.`. This Nadya program would crash:

```
...
1_W
...
```

But could be fixed like:

```
.....
.1_W.
.....
```

## Belts

Belts can't run parallel to one another. This is a current limit of the language and may be resolved in a future version.

```
.....                  .....
.v<<.  <-- directions  .___.
.>>>.  belts -->       .___.
.....                  .....
```
