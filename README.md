# RandomStringUtils/nextInt Cracker

This is a tool to crack [RandomStringUtils](https://commons.apache.org/proper/commons-lang/apidocs/org/apache/commons/lang3/RandomStringUtils.html) and Java's default `java.util.Random.nextInt(bound)` when `bound` is odd. See the accompanying [blog post](https://elttam.com/blog/cracking-randomness-in-java/) for some background and how it works.

## Installation

Download and build the tool from source:

```sh
git clone https://github.com/elttam/rsu-cracker.git
cd rsu-cracker
cargo install --path .
```

## Usage

```
Usage: rsu-cracker [COMMAND]

Commands:
  random-alphanumeric
  next-int
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```
Usage: rsu-cracker random-alphanumeric [OPTIONS] <TOKEN>

Arguments:
  <TOKEN>  Output of RandomStringUtils.randomAlphanumeric(n)

Options:
  -c, --count <COUNT>            Number of tokens to output [default: 10]
  -o, --output-len <OUTPUT_LEN>  Length of tokens to output [default: 0]
  -h, --help                     Print help
```

```
Usage: rsu-cracker next-int [OPTIONS] --n <N> [OUTPUTS]...

Arguments:
  [OUTPUTS]...  Outputs of random.nextInt(n)

Options:
  -n, --n <N>          Value of the bound n
  -c, --count <COUNT>  Number of values to output [default: 10]
  -h, --help           Print help
```

## Examples

```sh
❯ rsu-cracker random-alphanumeric -c4 ygZDDnsAqDNRHT4FS5UrHN3vi6mEGTJr
[*] Attempting with skips = (0, 0, 0)
100%|███████████████████████████████████████████| 23598721/23598721 [00:05<00:00, 4467906.00it/s]
[*] Attempting with skips = (0, 0, 1)
100%|███████████████████████████████████████████| 23598721/23598721 [00:05<00:00, 4319475.50it/s]
[*] Attempting with skips = (0, 1, 0)
100%|███████████████████████████████████████████| 23598721/23598721 [00:05<00:00, 4287969.00it/s]
[*] Attempting with skips = (1, 0, 0)
100%|███████████████████████████████████████████| 23598721/23598721 [00:05<00:00, 4212314.50it/s]

[*] Finished running after 23.634s
[+] Java Random seed recovered: 1668247404
[+] The next 4 tokens are:
gcFPzMAEVIxlrpYx4NZedHoLFgjgHTeF
6KjEP8eTsNne85XWSgXaHS9C1rwmxbKI
dMZ0o9XO8RvTcoHjUT4WWcCUAUeehxmj
vgrdwxLmgzC4nUoZCw7CWi0hCMHbi3oA
```

```sh
❯ rsu-cracker random-alphanumeric -c1 -o 50 GxZRXq7BWQ
[*] Attempting with skips = (0, 0, 0)
100%|███████████████████████████████████████████| 23598721/23598721 [00:05<00:00, 4403258.50it/s]

[*] Finished running after 5.597s
[+] Java Random seed recovered: 175138514385774
[+] The next token is:
n103Z9XtEiGqQnwxh4ifc6dC2tjRaV7nm8Qi93cW7Z3QTOGJlK
```

```sh
❯ rsu-cracker next-int -n7 -c5 0 2 2 0 5 6 5 6 5 2 4 4 4 5 5 4 2 4 6 5
100%|███████████████████████████████████████████| 306783378/306783378 [02:50<00:00, 1798661.75it/s]

[*] Finished running after 362.326s
[+] Java Random seed recovered: 7955809
[+] The next 5 outputs are:
3
4
2
5
2
```

```sh
❯ rsu-cracker next-int -n1337 -c7 509 287 813 384 1013 1254
100%|███████████████████████████████████████████| 1606195/1606195 [00:00<00:00, 4161011.50it/s]

[*] Finished running after 0.45s
[+] Java Random seed recovered: 1337
[+] The next 7 outputs are:
1317
623
1123
457
201
1176
640
```

## Known Issues

- The `next-int` command is not as effective for very small bounds (i.e. less than 7). In some cases it may not be able to recover the seed.
- If the `random-alphanumeric` command is not able to crack a string that you're confident comes from RandomStringUtils, try reversing the string as some older versions of RandomStringUtils build the string in reverse order.
