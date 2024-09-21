# Orion-Shield
This is a CLI program that generate passwords

## Getting started
```bash
# generate default password
cargo run --bin orion_shield

# go interactive mode (-- are required to enable flags)
cargo run --bin orion_shield -- -i
```

## Usages
### Interactive mode
```bash
# pass the -i flag to get interactive mode
orion_shield -i
```
Interactive mode give you a menu where you can interact with the program. 
- updating password configuraiton (length, uppercase...)
- generating as many password as you want

### CLI mode
There are 3 flags that you can use to generate one password:
- **-l** set your password's length (max 255)
- **-u** add uppercase to the password
- **-d** add digit to the password
- **-c** add special characters to the password
By default the password configuration has a length of 20, lowercases only.
```bash
# run with default config
orion_shield

# run with specified length and digit enabled
orion_shield -l 12 -d

# run with every config enabled (and a default length)
orion_shield -d -c -u

# run with every config enabled with specified length
orion_shield -l 50 -d -c -u

```
