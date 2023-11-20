# Multiple Console Applications

This is a series of utilities writen in rust, with the objective of training the language and it's syntaxes

## Basic Usage

```console
cargo run --bin <implementation-name>
```

## Created Implementations

| Implementation Name | Original functionallity |              Description           |
|---------------------|-------------------------|------------------------------------|
|            my-cat   |         cat             |         concatenates files         |
|            my-grep  |         grep            |       matches texts in files       |
|            my-echo  |         echo            |            repeats input           |
|            my-ls    |          ls             |           lists directories        |
|            my-find  |          find           |   locates files or directories     |

## my-echo

usage:

if builded:

```console
./target/release/my-echo "my text"
```
else:

```console
cargo run --bin my-echo -- "my text"
```
## my-cat

usage:
if builded:
```console
./target/release/my-cat "path/to/my/file"
```
else:
```console
cargo run --bin my-cat -- "path/to/my/file"
```
