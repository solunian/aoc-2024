# aoc-2024

my advent of code solutions for 2024... im such a rust newbie

## day03
why the hell are strings so weird in rust??? Ok. it makes sense with bytes and chars and graphemes or wtvr.
but... that just makes me have to think harder about stuff. wth. Also results and options have so many different
ways of resolving them. unwrap and unwrap_or and augh. the flip.

## day04
ahhhhhhhhhhh my part 1 was so terrible. idk if complete search is the most efficient.
coding the conditionals was annoying, especially with weird rust stuff.

## day05
macros are quite something. rust has no varargs or overloading which is interesting.
borrowing is also kinda interesting when your code works.

## day06
wth. tried impl with Add Trait and the code is certainly something.
in part 2, i did a bad brute force version which looped through a vec for finding a cycle which was already O(n^3). yea i know its bad.
then i used a HashSet with all the path configs which was much faster with O(n^2). my solution took 15 seconds :skull:. 
idk if theres a better way of doing it.
bryan was pretty smart; just count if the traveled count goes greater than the number of cells available.
rust breaks my brain. i might be dumb.

## day07
used the crate itertools. worked pretty well. should also practice using bitshiting and bitwise stuff 
for permutations/subsets and all that, but that's for another time. im lazy rn. also generating with recursion/iteration.

## day09
was being a little dum dum for day09