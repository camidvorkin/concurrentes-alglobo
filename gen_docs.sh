#!bash

cargo doc --no-deps --target-dir docs --open

# We first remove every sequence of '//! '
# We also remove every sequence of '//!' to account for line skips
# We also rectify the img path to be relative to the root of the repo in the PDF, but not in the rustdoc
# We also remove the dummy main method
cat src/informe.rs  | sed 's/\/\/!\ //g' | sed 's/\/\/!//g' | sed 's/..\/..\/img/docs\/img/g' | sed 's/fn main() {}//g' | pandoc -f markdown --metadata-file pandoc_headers.md -o docs/informe.pdf
