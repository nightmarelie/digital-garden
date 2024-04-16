# Digital garden

A CLI tool for creating and managing a digital garden.

## Commands


### write

```bash
garden write
garden write -t "Title"
garden write -t "Title" -c "Content"
```

# Development

## Installation

```bash
cargo install cargo-watch # Install cargo-watch to watch for changes
watch -x 
watch -x "cargo build" # Run cargo build on file changes
```

### Setting the garden path

```bash
export GARDEN_PATH=~/path/to/garden
garden -p $GARDEN_PATH write
garden --garden_path $GARDEN_PATH write

# or
GARDEN_PATH=~/path/to/garden garden write
```

