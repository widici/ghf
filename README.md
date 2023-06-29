### Table of content
1. [About](#about)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Licensing](#licensing)

### About
Ghf standing for github fetch is a cli tool written in `rust` that fetches and displays github user information kind of like [neofetch](https://github.com/dylanaraps/neofetch).

### Prerequisites
- [Rust 2021](https://www.rust-lang.org/tools/install)

### Installation
Installation with Cargo:
```
cargo install --git https://github.com/widici/ghf
```

### Authentication
You can optionally provide an PAT (personal access token) to raise the request limit from 60 to 5000 requests per hour.
To authenticate first generate a new PAT without permissions at [developer settings](https://github.com/settings/tokens) and then run the following command:
```
ghf auth <token>
```

### Usage

```
ghf <username>
ghf <username> --color <color> # Print the text in a specific color
```

For more information about the commands run the following command:
```
ghf --help
```

### Licensing
This repository is dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
