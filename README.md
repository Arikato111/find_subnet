<p align="center">
    <img height="30" width="40" alt="rust" src="https://github.com/Arikato111/Arikato111/raw/main/icons/rust.svg">
</p>

# <p align="center">Find subnet mask</p>

<p align="center">
    <img alt="Github License" src="https://img.shields.io/github/license/Arikato111/find_subnet" />
    <img alt="GitHub Issues" src="https://img.shields.io/github/issues/Arikato111/find_subnet" />
    <img alt="GitHub Pull Requests" src="https://img.shields.io/github/issues-pr/Arikato111/find_subnet" />
    <img alt="GitHub Contributors" src="https://img.shields.io/github/contributors/Arikato111/find_subnet" />
    <img alt="GitHub Last Commit" src="https://img.shields.io/github/last-commit/Arikato111/find_subnet" />
    <img alt="" src="https://img.shields.io/github/repo-size/Arikato111/find_subnet" />
</p>

console application for find subnet mask

### Install (For linux)

1. run code bellow on your terminal with `curl` or `wget`

```bash
curl -o- https://raw.githubusercontent.com/Arikato111/find_subnet/main/release/install.sh | bash
```

```bash
wget -qO- https://raw.githubusercontent.com/Arikato111/find_subnet/main/release/install.sh | bash
```

### Build

- `cargo build --release`
- output file is `./target/release/find_subnet`

### Run

- `subnet <0-32>`
- `subnet <0-32> <0-32> <0-32> <0-32>`

### example

```bash
$ subnet 20
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
```

```bash
$ subnet 10 20 30
# /10 = 11111111.11000000.00000000.00000000 | 255.192.0.0
# /20 = 11111111.11111111.11110000.00000000 | 255.255.240.0
# /30 = 11111111.11111111.11111111.11111100 | 255.255.255.252
```

#### find wildcard

```bash
$ subnet -w 10 20 30
# /10 = 00000000.00111111.11111111.11111111 | 0.63.255.255
# /20 = 00000000.00000000.00001111.11111111 | 0.0.15.255
# /30 = 00000000.00000000.00000000.00000011 | 0.0.0.3

```

### TODOs

- [ ] find subnet with number of devices 
- [ ] update to latest version
- [x] find wildcard
- [ ] installing. use git clone but dont move any file or folder. just git clone.