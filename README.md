this is my first ever program written in Rust; pfhew, it took a while to get to build !

so what the program does is

* build the complete graph with 9!/2 nodes each representing a reachable board
* scan the graph using the dijkstra algorithm, starting with the target board

so at the end we have all the optimal distances - and paths - for
every of those reachable boards, that takes about 2.2s to compute
(with the release build of course); a histogram of the various
distances is printed out; the 2 worst boards are 31-moves away from
the target board, as stated here
<http://w01fe.com/blog/2009/01/the-hardest-eight-puzzle-instances-take-31-moves-to-solve/>

if filenames are provided on the command line, the solution for those boards are printed out



