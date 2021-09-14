# host2e

A simple hosted image encoder.

Encode specified image hosted online, and store it in text file.

# How to run this

```Shell
#cargo run photoURL -t encode_type
cargo run https://cdn.pixabay.com/photo/2020/02/29/18/59/rabbit-4890861_960_720.jpg -t base64
```

then host2e.txt will be generated at the top level directory.

# Available types

- `base64`