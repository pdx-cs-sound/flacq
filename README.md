# flacq: Free lossless audio compression by quantile
Bart Massey 2022

This is a quick-and-dirty implementation of the
dumbest-possible use of the excellent
[`q_compress`](https://crates.io/crates/q_compress)
"Quantile Compression" scheme to compress audio files.

At the moment the compressor/decompressor reads `stdin` and
writes `stdout`. It has one free parameter (delta encoding
order) with a reasonable default. Use `--help` for
invocation details.

The compression performance is worse than `FLAC`, but
usually not dramatically worse. *Any* level of finesse in
the use of quantile compression (for example, using it
primarily for residue encoding) would likely make this quite
a strong compressor.

This work is made available under the "Copyleft Next License
version 0.3.1". Please see the file `LICENSE.txt` in this
distribution for license terms.
