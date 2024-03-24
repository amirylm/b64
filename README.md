# b64

A simple CLI for encoding or decoding base64 strings.

## Usage

**Encoding**

```bash
b64 e --help

# Encode data
# Usage: b64 encode [OPTIONS]
# Options:
#   -f, --file <file>    File containing data to encode
#   -i, --input <input>  Raw string input
#   -h, --help           Print help

# Examples:
#   b64 e -f config.toml
#   b64 encode -i "test"
```

**Decoding**

```bash
b64 d --help

# Decode data
# Usage: b64 decode [OPTIONS]
# Options:
#   -f, --file <file>    File containing data to decode
#   -i, --input <input>  Raw string input
#   -h, --help           Print help

# Examples:
#   b64 d -f config.toml
#   b64 decode -i "dGVzdA=="
```
