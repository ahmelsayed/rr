A simple command line key-value store based on [boom](https://github.com/holman/boom) written in Rust.

I am only writing this as a way to learn Rust, so excuse the poor code as I learn.

Possible inputs:

    get:
        rr {key}

    set:
        rr {key} {value}

    commands:
        rr ?
        rr delete {key}
        rr all