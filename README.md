# wolffserial
![GitHub Build_Win](https://img.shields.io/github/workflow/status/wolffshots/wolffserial/build_win/main)
![GitHub Build_Ubuntu](https://img.shields.io/github/workflow/status/wolffshots/wolffserial/build_ubuntu/main)
![License](https://img.shields.io/github/license/wolffshots/wolffserial)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/wolffshots/wolffserial)
![GitHub repo size](https://img.shields.io/github/repo-size/wolffshots/wolffserial)
![Lines of code](https://img.shields.io/tokei/lines/github/wolffshots/wolffserial)
![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/wolffshots/wolffserial)

a lil helper program for reading from a serial port and listing available serial ports

## building and developing

### windows
as long as you have cargo on the path you should be able to build with `cargo build` or run with `cargo run`

### linux
you may need to install `libudev-dev` and `pkg-config` to build the package properly 
on ubuntu this would most likely be done with:
```bash
# apt install libudev-dev pkg-config
```

## running the binary
the binary needs to be run from a terminal with a subcommand to tell it what to do.
the available options and commands can be displayed by running with the `help` subcommand or `--help` flag and this can be used to see help for a specific command as well (such as `./wolffserial help watch` to see the help for the `watch subcommand`)

below is an example output from running `./wolffserial help` and from this you can see the functions of the subcommands
```
Recieve input from a serial device 
Reads data from a serial port and echoes it to stdout

USAGE:
    wolffserial [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    list     Lists available devices
    watch    Watch a specific device
```
