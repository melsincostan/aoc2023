# aoc2023

Code and notes about trying to learn rust using AoC 2023 puzzles // started late on 10.12.2023

**WARNING**: This contains spoilers for the respective Advent of Code puzzles. Consider implementing them yourself before checking my (very bad and ugly, probably) code out!

## Thoughts
### Day 1

- Part 1 seems like it is decent enough, it looks somewhat like rust code probably?
- Part 2 is bad, that if / else is way too long, i should find something better
- Using tests seems like a good habit to have?

### Day 2

- Part 1: maybe that's a bit too many functions? Solve works with chained `map()` calls which is cool
- Part 2: can't have multiple `map()` calls after one another for some reason? That can't be good

### Day 3

- These algorithms feel like they are horrendously unoptimized and very not rust like

### Day 4

- Part 1: feels more rust-ish again?
- Part 2: bit less rust-ish, lost time overcomplicating it w/ HashMap when a regular Vec is better since these were contiguous ints.
- Part 2: Messed around a bit with figuring out how to add stuff properly, having tests helped

### Day 5

- Part 1: IDK how rustish this is
- Part 1 could be parallelized the same way as part 2 but it's already pretty fast
- Part 1/2 i am such a dummy with the `>=` instead of `>` for the second part of the range check
- Part 2: Initially attempted with just caching using a HashMap. That might have been slower, added complexity, was abandonned
- Part 2 parallelized works fast enough (~4 minutes debug / ~10s release) with low ram (seemingly around ~3-4 MB on a dev build thanks to already doing a round of `.min()` on the parallel loop to slash the amount of possible locations to the local minimum)
- Bruteforcing isn't very smart but working with `rayon` was fun

### Day 6

- Part 1: definitely could have bruteforced this pretty easily
- Part 2: probably could have bruteforced this as well with parallelization
- Solving the polynomial is way easier, and there aren't really any edge cases to worry about in that situation

### Day 7

- Part 1: It was fun, although idk how idiomatic my code is again :c
- Part 2: My optimizer function is way overkill (i could have just returned the `Hand` it optimizes to), it is non-deterministic because i have no guarantees about the order of keys in an `HashMap` so i can't really test it and it is too big and complex but it works :3
- Part 2: Should have written tests for card power comparison, forgot to demote the power of the Joker at first and i thought i had a bug in my optimizer (maybe there is one but now i return the right result hehe)