# FPS Lang

A "frames per second" meme language 👌

The concept of this language is to execute statements on a *per frame* level.

This is based on the book [Crafting Interpreters](https://craftinginterpreters.com/)

## Logic

### Frame concept

\# represents a frame

```
# let a=1; # a; ## <- exit program on last frame
^       ^
|       |_frame 1
frame 0
```

This will **not work** as the variable 'a' assignment was requested on frame 0.
We need to wait as time progresses to interact with actions that are requested at a particular frame.
```
# let a=1; a; ##
        ^
        |
```

### Syntax

| Syntax                    | Description           | Example                       |
| ------------------------- | --------------------- | ----------------------------- |
| let id=\<val>;            | variable desclaration | # a let = 0; ##               |
| #2                        | 2 frames              | #2 print("hello\n"); ##       |
| for 0..2 { <statements> } | for loop              | # for 0..2 { println(it);} ## |

#### Example outputs

##### Multiple frames
```
#2 print("hello"); ##

output:
hello
hello
```

```
let a= 0; #5 a+=1; println(a); ##

output:
1
2
3
4
5
```

##### For loop

```
for 0..=2 { println(it); } ##

output:
0
1
2
```

And here is where it gets weird 🤣
```
for 1..=3 { print("\nframe {}", it); } # print(" - another print in same frame '1'\n") ##
^           ^                          ^
|           |                          |_ frame 1
|           |_this will start being executed on the next frame (1)
frame 0


output:

frame 1 - another print in same frame '1'

frame 2
frame 3
```
