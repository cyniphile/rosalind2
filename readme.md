__Rosalind Problems in Rust__

To start a new problem, run the following in the root project dir:

```sh
$ cargo new --lib {problem_name}
```

Add the lib as a dependency in the root `Cargo.toml`

```toml
...
members = [
	"fib",
	"tools",
	"gc",
	"subs",
	"cons",
	"{problem_name}"
	]
```

In the `{problem_name}` directory, add the `tools` crate to the `Cargo.toml` to get access to reusable helper functions:

```toml
...
[dependencies]
tools = {path = "../tools/"}
```

