# Turtle
A Generative Art tool in Rust!

This was inspired by [Isaacg1's Programatically Generated Artwork](https://isaacg1.github.io/2018/12/06/programmatically-generated-artwork.html).

## Build & Run

### Installation via cargo

Install on any platform using Cargo:

```console
$ cargo install --git https://github.com/wcygan/turtle
```

### How to run
```console
$ turtle -s <size> -n <name> -p <pattern> -r <rng>
```
### Example
```console
$ turtle --size 200 --name foo --pattern square --rng 99995
```
## Options

The program options that can be used:

| Option      | Usage                                                                       | Example            |
| :---------- | :---------------------------------------------------------------------------| :----------------- |
| `-s`        | The number of pixels, N, to create an N x N image                           | `-s 10`            |
| `-r`        | The seed used to initialize a pseudorandom number generator                 | `-r 1234`          |
| `-n`        | The name of the output file                                                 | `-n foo`           |
| `-p`        | The image pattern to use                                                    | `-p square`        |


### Supported patterns:
- Square

### Patterns to be implemented soon:
- Circle
- Fractal
- ... more complex patterns!

## Dependencies
- [clap](https://docs.rs/clap/2.33.3/clap/) to parse commands
- [image](https://docs.rs/image) to create images
- [rand](https://docs.rs/rand) to generate pseudorandom numbers
- [rayon](https://docs.rs/rayon/1.5.0/rayon/) to parallelize execution

## Output Examples

### Square
```console
$ turtle --size 200 --name foo --pattern square --rng 99995
```
![alt text](examples/foo.png)