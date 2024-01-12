# aoc2023

Code and notes about trying to learn rust using AoC 2023 puzzles // started late on 10.12.2023

**WARNING**: This contains spoilers for the respective Advent of Code puzzles. Consider implementing them yourself before checking my (very bad and ugly, probably) code out! The thoughts will probably contain spoilers as well!

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

### Day 8

- Part 1: Doesn't feel super idiomatic somehow
- Part 2: First tried to bruteforce, then figured out the math way, then i spent way too long trying to implement my own lcm that works on the array directly using the wikipedia algorithm only for it to take way too long on the actual input so i used the LCM from a crate and made a recursive function to get the lcm of an array, since there are only 6 elements this isn't an issue

### Day 9

- Both parts could have used some more testing...
- Part 1: Probably not idiomatic, it works though
- Part 2: IDK if i can push to the beginning of a `Vec` and i don't want to figure it out. I can take the part 1 code, reverse the array and replace the `+` by a `-` anyways and it's good enough for me.

### Day 10

- Part 1: Pretty ugly, works decently enough, didn't write tests
- Part 2: Tried flooding (doesn't work, because one can squeeze between pipes)
- Part 2: Tried raycasting (didn't take into account avoidable pipes, failed)
- Part 2: considered using strings method to get the area, was unsure whether it would work
- Part 2: implemented better raycasting ported from https://www.youtube.com/watch?v=r3i3XE9H4uw

### Day 11

- Part 1: Rotating 2D arrays yay
- Part 2: I should have done this for part 1 as well, this expansion is a lot more elegant i think? it might also take less memory...

### Day 12

- Part 1: actual solving logic (recursion) from https://www.youtube.com/watch?v=g3Ms5e7Jdqo
- Part 2: used the same optimization as the video (memoization) to deal with the larger values

### Day 13

- This was fun, after the frustration of not figuring out day 12 and of figuring out a potentially correct raycaster for part 2 of day 10 (My problem was casting a ray to the top-left corner of each tile, which would end up in edge cases with L-shapes in the loop at that point. I might go around and reimplement it someday).
- Part 2: Having tests helped out with debugging.
- Part 2: I'm decently happy with that solution. It feels somewhat elegant and doesn't seem to be adding /too/ much complexity

### Day 14

- Part 1: This was fun to implement, especially the sliding the rocks around!
- Part 2: Tried with memoization first, it was too slow
- Part 2: Solved by finding a cycle of weights repeating, it has to repeat at least 4 times, with the series saved up to that point and a HashMap to have occurences of each weight. The final weight can then be calculated with the position in the cycle of the amount of iterations and the start offset of the cycle.

### Day 15

- Part 1: This was a breeze to implement (not sure how idiomatic my code is, still...) and pretty fun!
- Part 2: I like hash maps, implementing one was super fun :3 It isn't the greatest thing ever in terms of hash maps, but the puzzle also somewhat pushed to this i guess

### Day 16

- Part 1: Again doesn't feel very idiomatic
- Part 2: I thought this would need rayon, but running a release build is barely more than a few seconds. Rayon makes it instant. This is why i used map to compute everything, if i decide i need to parallelize i can just pull in rayon and replace `iter()` with `par_iter()`

### Day 19

- Part 1: There is no way this is idiomatic or proper code... It has tests tho???