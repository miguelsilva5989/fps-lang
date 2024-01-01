# FPS Lang

A "frames per second" meme language to learn how to develop a compiler.

## Logic

### Frame concept

\# represents a frame

```
# a:=1; # a; # <- exit program on last frame
^       ^
|       |_frame 1
frame 0
```

This will **not work** as the variable 'a' assignment was requested on frame 0.
We need to wait as time progresses to interact with actions that are requested at a particular frame.
```
# a:=1; a; #
        ^
        |
```

### Syntax

| Syntax                    | Description           | Example                      |
| ------------------------- | --------------------- | ---------------------------- |
| var:=\<val>;              | variable desclaration | # a := 0; #                  |
| 2#                        | 2 frames              | 2# print("hello\n"); #       |
| for 0..2 { <statements> } | for loop              | # for 0..2 { println(it);} # |

#### Example outputs

##### Multiple frames
```
2# print("hello"); #

output:
hello
hello
```

```
# a:= 0; 5# a+=1; println(a); #

output:
1
2
3
4
5
```

##### For loop

```
# for 0..=2 { # println(it); # } #

output:
0
1
2
```

And here is where it gets weird 🤣
```
# for 1..=3 { println("frame {}", it); } # print(" - another print in same frame '1'") #
^             ^                          ^
|             |                          |_ frame 1
|             |_this will start being executed on the next frame (1)
frame 0


output:
frame 1 - another print in same frame '1'
frame 2
frame 3
```
