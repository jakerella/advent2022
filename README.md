# Advent of Code 2022

This is my solution(s) for the Advent of Code challenge, 2022!

This year I've done the challenge in [Rust](https://www.rust-lang.org), which I have never coded in before. Fun!

To run this repo (and thus the code challenge solutions):

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. If you're in WSL (or just a vanilla linux install), install the build tools: `sudo apt install build-essential`
3. Clone this repo
4. Run (and compile) the code for a given day:

`cargo run -- --day 1 --input-file ./src/inputs/day1.txt`

You can ignore the second param (`input-file`) and the script will look for a "dayX.txt" file in the `src/inputs` directory. This is just a shortcut.

`cargo run -- --day 1`

The two commands above are equivalent.
