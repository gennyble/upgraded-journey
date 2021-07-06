# arcdrop
When are `Arc`s dropped? How do they really work? I was unsure about `Arcs`, so I
thought I'd try and figure it out. 

There isn't really anything particularly exciting here. I just wanted to see if I
knew how `Arc`s worked. The documentation says how they work, but I guess I needed
to use them myself to be sure.

The code in `src/main.rs` should print as below, or similar:
```
[task1] levi says: mrrrow
[task3] genevive says: mrrrow
[task2] levi says: mrrrow
dropped levi
[task4] genevive says: mrrrow
dropped genevive
```