# Facade attribute

This is an experimental implementation of the `inline` keyword alternative as discussed on the [Rust internals forum](https://internals.rust-lang.org/t/pre-rfc-inline-mod/5716).

The custom attribute is intended to simplify the module facade pattern which is common to rust module structure.
It reexports the `pub` items of the annotated module.
```Rust
#[facade] mod sub_module;
```

It supports a *pseudo hiding* feature to make the annotated module unaddressable by its name.
This is done by mangling the modules name with the time of compilation.
The hiding might fail if by chance another item with the mangled name is already defined (or if the time of compilation can be predicted).
```Rust
#[facade(hide)] mod sub_module;
```

Have a look in the `tests` folder to see some example usage.
This attribute requires `proc_macro_attribute` and is currently only usable on the **nightly** channel of Rust.
