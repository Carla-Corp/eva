<img align="right" src="./assets/eva.png" alt="EVA Logo" width="120px" height="120px">
<br><br>

# 🍎 EVA
### Declarative Configuration Language

EVA is a human-readable declarative configuration language focused on simplicity, composition and readability.

It was designed to provide a cleaner and more expressive alternative for application configuration, while remaining easy to parse, write and maintain.

```eva
@project
name: "project name"
description: "that is an awesome project"
version: "3.20"

@author
name: "Jane Doe"
contact: {
    phone: "00 999 000000"
    email: "xxx@xxx.com"
}
````

# How does EVA work?

Like TOML, YAML and other declarative languages such as TOON, EVA is designed to store structured data that can later be read and processed by applications.

However, while languages like TOON focus on token optimization and TOML focuses on readability and simplicity, EVA aims to provide both a clean configuration format and lightweight runtime data interpretation.

This allows applications to dynamically resolve values, execute utility functions and compose data during initialization, making configuration management more practical and expressive for developers.

## What do I mean by practicality?

EVA provides built-in utility functions that simplify common configuration tasks.

For example:

```eva
@paths
home: env("HOME")
```

In this example, EVA resolves the user's `HOME` environment variable and stores the result inside `home` when the application starts.

EVA also includes many other utility functions and data composition features.
| Function | Description | Example |
|---|---|---|
| `absolute()` | Returns the absolute version of a path | `absolute("./config")` |
| `basename()` | Returns the file name from a path | `basename("/home/file.txt")` |
| `clamp()` | Restricts a number between a minimum and maximum value | `clamp(volume, 0, 100)` |
| `coalesce()` | Returns the first non-null value | `coalesce(a, b, c)` |
| `contains()` | Checks whether a string, array or map contains a value | `contains(items, "apple")` |
| `debug()` | Prints or exposes internal debug information | `debug(config)` |
| `deep_merge()` | Recursively merges maps | `deep_merge(base, user)` |
| `else()` | Returns a fallback value if the first value is null | `else(port, 8080)` |
| `ends_with()` | Checks whether a string ends with a value | `ends_with(name, ".png")` |
| `entries()` | Returns all key-value pairs from a map | `entries(user)` |
| `env()` | Reads environment variables | `env("HOME")` |
| `extname()` | Returns the extension of a file | `extname("image.png")` |
| `format()` | Formats a string using placeholders | `format("Hello {}", name)` |
| `if()` | Returns one of two values based on a condition | `if(debug, "yes", "no")` |
| `important()` | Marks a value or section as important | `important(token)` |
| `keys()` | Returns all keys from a map | `keys(config)` |
| `lower()` | Converts text to lowercase | `lower(name)` |
| `merge()` | Merges maps shallowly | `merge(a, b)` |
| `ref()` | References local values inside the current namespace | `ref(home)` |
| `starts_with()` | Checks whether a string starts with a value | `starts_with(path, "/home")` |
| `trim()` | Removes leading and trailing whitespace | `trim(input)` |
| `upper()` | Converts text to uppercase | `upper(name)` |
| `values()` | Returns all values from a map | `values(config)` |

## Language Support

EVA is currently under active development.

The parser, evaluator and standard utility functions are being designed to remain portable and easy to implement across multiple programming languages and runtimes.

The goal is to provide native EVA implementations and libraries for several ecosystems while maintaining consistent behavior between them.

| Language | Status |
|---|---|
| C | In development |
| C++ | In development |
| Rust | In development |
| Go | Planned |
| Zig | Planned |
| JavaScript / TypeScript | Planned |
| Python | Planned |
| Java | Planned |

Additional language bindings and implementations may be added in the future.

The comunity also are welcome to make some language bindings.  

