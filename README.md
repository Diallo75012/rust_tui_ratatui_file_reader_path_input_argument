# Rust Custom File Reader TUI

this project is done as exploration of TUI creation in `Rust`
It was part of the kubernetes cluster upgrade TUI project but I have decided to put it on the side (on its own repo)
- it take as `cli` input argument the path of the file to read
- can use arrow `up/down` or `k/j` to move `up/down` by one line, `pgup/pgdown` to move `up/down` by 10 lines.
- use `q` to quit.

I have tried to see also how to create custom error handling, and have put that in it's own crate file `error.rs`

To start it:
- just run it using: `cargo run <path to the file to read>`


I will reuse some of those concepts in my Rust Kubeadm Uprgade Cluster project.
