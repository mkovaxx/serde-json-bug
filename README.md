# serde-json-bug

This snippet reproduces a bug in Rust's `serde_json` crate.

Expected output:
```
Err(Error("unknown variant `xenomorph`, expected `cat` or `dog`", line: 6, column: 34))
```

Observed output:
```
Err(Error("unknown variant `xenomorph`, expected `cat` or `dog`", line: 0, column: 0))
```
