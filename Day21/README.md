# Day21

```
part 1: 125742 (88.80µs)
part 2: 157055032722640 (955.90µs)
```

## algorithm

top down dynamic programming with caching/memory/memorization

part 1 uses a loop

part 2 uses recursion

## notes/experimentation

bottom is truth

```
    <A^A>^^AvvvA
    <A^A>^^AvvvA

    v<<A>>^A<A>AvA<^AA>A<vAAA^>A
    v<<A>>^A<A>AvA<^AA>A<vAAA>^A

    <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA^>Av<<A>^A>AAvA^Av<<A>A^>AAA<Av>A^A
    <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
    
    029A: 68 * 29

    ^^^A<AvvvA>A
    <AAA>Av<<A>>^A<vAAA^>AvA^A
    v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A^>AAA<Av>A^A<vA^>A<A>A
    980A: 60 * 980

    ^<<A^^A>>AvvvA
    <Av<AA>>^A<AA>AvAA^A<vAAA^>A
    v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A
    179A: 68 * 179

    ^^<<A>A>AvvA
    <AAv<AA>>^AvA^AvA^A<vAA^>A
    v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA^>A<A>A<vA^>A<A>Av<<A>A^>AA<Av>A^A
    456A: 64 * 456

    ^A^^<<A>>AvvvA
    <A>A<AAv<AA>>^AvAA^A<vAAA^>A
    v<<A>>^AvA^Av<<A>>^AA<vA<A>>^AAvAA<^A>A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A
    379A: 68 * 379
```