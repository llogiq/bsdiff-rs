# bsdiff-rs
A Rust BSDiff port

This initially aims to be compatible with
[M. Endsley's BSDiff implementation](https://github.com/mendsley/bsdiff)
because it lets us test the diff and patch tools indenpendently. However, an
improved format using
[Dropbox' brotli implementation](https://github.com/dropbox/rust-brotli) for
even better compression is on the roadmap.

TODO:

* [x] working rspatch (bspatch)
* [ ] Pick a license
* [ ] working rsdiff (bsdiff)
* [ ] better error handling
