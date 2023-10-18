## Task

The calculator can perform operations of addition, subtraction, multiplication and
division with two numbers: a + b, a - b, a * b, a / b.

The calculator must accept numbers from 1 to 10 inclusively, no more.
At the output, the numbers are not limited in size and can be any
(Roman (I, II, III, IV, V ...) numbers).

The calculator can only work with whole numbers.

## Usage

```console
$ cargo run 1 + 2
3

$ cargo run X + 2
12

$ cargo run V - 9
-4

$ cargo run VI / II
3

$ cargo run 10 \* 10 # one need to escape asterisk
100

$ cargo run X '*' 10 # or put it in quotes
100

$ cargo run 1 2 3
invalid argument '2', operator (+ - * /) expected

$ cargo run - 2 3
invalid argument '-', arabic or roman number from 1 to 10 expected

$ cargo run 1 - 3 + 1
exactly 3 arguments expected, 5 provided instead
```
