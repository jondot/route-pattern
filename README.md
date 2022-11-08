Route Pattern
=============

[<img alt="github" src="https://img.shields.io/badge/github-jondot/route_pattern-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/jondot/route-pattern)
[<img alt="crates.io" src="https://img.shields.io/crates/v/route-pattern.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/route-pattern)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-route_pattern-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/route-pattern)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/jondot/route-pattern/Build/master?style=for-the-badge" height="20">](https://github.com/jondot/route-pattern/actions?query=branch%3Amaster)

A parser and matcher for a popular way to create route patterns.

Patterns like these that include regular expressions, delimited in this case by `{` and `}`:


```
/users/{[0-9]+}/update
```

Supports nested curlies here (inner curlies are serving the regular expression):


```
/users/{[0-9]{1,8}}/update
```

It lets you:

* Choose your delimiters: `{`, `}` or `<`, `>` or others
* Compile into a `Regex` or try a match

## Dependency

```toml
[dependencies]
route-pattern = "0.1.0"
```

For most recent version see [crates.io](https://crates.io/crates/route-pattern)


## Usage

```rust
let answer = route_pattern::is_match("foo/{b{1,4}}/{[0-9]+}", '{', '}', "foo/bbb/123")?
```

Or get a `Regex` and use it later

```rust
let re = route_pattern::compile("foo/{b{1,4}}/{[0-9]+}", '{', '}')?;
```

# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.






















