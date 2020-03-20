# import-of-call
Examples of Rust module imports.

Let's create an exmaple of as many different cases of rust module definitions and imports as possible. There are a bunch of ways to create and use a module so this is going to get confusing.

# Binary and Library Crates

First off there are two different kinds of crates, binary and library. This is well covered in The Book. To use code across this boundery(e.g. use code from the library in the binary) you need to specify the imports as if they come from a different create, albeit one with the same name. You can see this by using the `dock` module from `main`.