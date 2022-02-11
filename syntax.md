# Syntax Documentaiton of Rawk

## Hello World

```
fn main() {
    out!("Hello World");
}
```

In this case the ``!`` marks a function that is built-in into the interpreter.

## Typing

Rawk uses static typing, therefore every variable has to be a certain type at declaration. The type can't change at runtime.

Possible types:
 * String: "test"
 * char: 't'
 * int (Signed 64-bit integer): 13
 * double (64-bit): 13.0
 * bool: true/false
 * null

Rawk also has a garbage collector.

## Expressions

All the expressions are the same as in C. All comparisons are type safe.

## Variables

Variables can be declared like in C. Example:

```
int num = 5;
String str = "test";
bool isTrue = false;
```

## Control flow

Same as in C and furthermore you can iterate over an array by value using the ``in`` keyword.

## Functions

A function declaration uses the ``fn`` key word. A optional return value can be placed after the argument list.

```
fn sum(int a, int b) > int {
    return a + b;
}
```

## Classes

Rawk supports classes. All variables of the methods of an object are public.

```
class Test {
    String str;

    Test() {
        // constructor
    }

    fn printSomething(String s) {
        out!(s);
    }
}

fn main() {
    Test t = new Test();

    t.printSomething("hello");
}
```

## Examples

### Loop and If Example

```
fn main() {
    int a = 0;

    while (a < 10) {
        if (a % 3 == 0) {
            out!(a, " modulo 3 equals 0");
        } else {
            out!(a);
        }

        a++;
    }
}
```

### Array example

```
fn main() {
    int[] arr = [0, 1, 2, 3];

    for num in arr {
        out!(num);
    }
}
```

### Class example

```
class Test {
    String str;

    fn Test() {
        // constructor
    }

    fn compute(int num) > int {
        return num * 2 + 13 % 17;
    }

    fn format(int num) > String {
        return "number: " + num;
    }
}

fn main() > int {
    Test t = new Test();

    int ret = t.compute(13);

    String format = t.format();

    out!(format);

    return -1;
}
```
