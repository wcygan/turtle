# Turtle
A Generative Art tool in Rust

### Installation via cargo

Install on any platform using Cargo:

```console
$ cargo install --git https://github.com/wcygan/turtle
```

### How to run
`turtle -s <size> -n <name> -p <pattern>`

### Options

The program options that can be used:

| Option      | Usage                                                           | Example            |
| :---------- | :-------------------------------------------------------------- | :----------------- |
| `-s`        | Determines the size of the resulting image                      | `-s 10`            |
| `-n`        | Determines the name of the output file                          | `-n foo`           |
| `-p`        | Determines the image pattern to use                             | `-p circle`        |


### Dependencies
- [clap](https://docs.rs/clap/2.33.3/clap/) to parse commands
- [image](https://docs.rs/image) to create images