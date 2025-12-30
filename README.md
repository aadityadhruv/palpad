### Palpad

A really simple markdown *parser* + **static** site generator. 

It supports a small subset of the markdown spec. The goal is to be able to host
a directory of markdown files, and then use `palpad` to convert the directory
to valid HTML.

This is a code block:

```
int main() {
  int a = 1;
  int b = 2;
  string s = "Hello this is a **bold** statement. Here's a backtick` for you";
  int c = a + b;
  return c;
}
```

This, however, isn't ```a code block`.
