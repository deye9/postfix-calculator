# postfix-calculator

A postfix Calculator in Rust

## Usage

To compile, use ```cargo run```.
To use, enter integers and operators (+, -, *, /). Enter can be pressed in between inputs without exiting, assuming there are enough operands. To exit the program, press ```Control + D```. The final result will be output.

## Postfix Algorithm

foreach token
    if token is integer
        push token
    else if token is operator
        pop right side value
        pop left side value
        evaluate operator
        push result
    next

We will be using the stack to achieve this. Using the concept of LIFO.

### Example

```
cargo run "1 7 4 + 5 - *"
[Control + D]
```
will output
```
Result of calulation on "1 7 4 + 5 - *" is [ 6 ]
```

```
cargo run "5 6 7 * + 1 -"
[Control + D]
```
will output
```
Result of calulation on "5 6 7 * + 1 -" is [ 46 ]
```
