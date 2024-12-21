# Day18

```
part 1: 292 (1.64ms)
part 2: 58:44 (92.10ms)
```

## algorithm

dijkstra search from start to end.  keep track of previous path from start to end. if the new falling position was on the path compute a new path else the current path is still valid.  return the first falling byte's position that isn't a valid path.

## note 

part 2 prints out as `NN:NN` but you need to manually change the colon to a comma (`NN,NN`) before submitting the answer.