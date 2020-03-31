# postfix-calculator

A postfix Calculator in Rust

## Usage

To compile, use ```cargo run```.

To use, enter integers and operators (+, -, *, /).

Do space the values and remember to place them in quotes at the begining and end of the values. Once you press the enter key, the final result will be output.

## Postfix Algorithm

<pre>
  foreach token
    if token is integer
      push token
    else if token is operator
      pop right side value
      pop left side value
      evaluate operator
      push result
  next
</pre>

We will be using a vector [`std::vec::Vec`] to achieve this. Using the concept of LIFO.

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
