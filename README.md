## Find hash with trailing zeros


A console application that searches integers starting from 1, 
calculates a sha256 hash for each of them, 
and displays the hash and the original number on the console 
if the hash digest (character representation of the hash) 
consists of N null characters. 

The `-N` parameter is set by the user when the application is started. 

The `-F` parameter defines how many hash values the command should find.

#### Note
The application is executed only on the CPU

And just for test purpose 

#### Build

```bash
cargo build --release
```

#### Run

```bash
cargo run -- -N 3 -F 10
```

#### Example
```bash
$ cargo run -- -N 5 -F 3
828028, "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000"
2513638, "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000"
3063274, "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000"
```