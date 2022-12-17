# Focus:

`traits`, `impl` and `struct`

# Lesson 1: Structs and their public-ness

## Structs: Quiz

- [Official Docs](https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html) +

```rust
// What's the difference between all of the following?

pub struct PubStruct<T> {
    // case 1
    pub field: T
}

pub struct PubOuter<T> {
    // case 2
    field: T
}

struct PubInner<T> {
    // case 3
    pub field: T
}

struct PubNone<T> {
    // case 4
    field: T
}
```

## Structs: Solution

### PubStruct

The first method is public, so we can construct directly with the value names e.g

```rust
let a: PubStruct = PubStruct{field: - 1};
```

### PubOuter

whereas in the second one, we cannot construct the struct directly (bc of private field)

```rust

let b: PubOuter = PubOuter{field: 1} //results in error
```

How do we fix this?

```rust
impl<T> PubOuter<T> {
    pub fn new(some_entry: T) -> Self {
        PubOuter { field: some_entry }
    }
}

let b: PubOuter = PubOuter::new("Some value");
```

### PubInner

- realistically, nothing special here. You can't access the fields as the outer scope is private. BUT could be used to
  indicate.... something(?)

### PubNone

- can't be seen outside the cargo or whatever this is sitting in.

---

# Lesson 2: Impl and generic types

Say we have the goal of adding methods to some struct that we have, `Something`. We want to have two methods:

- a method specific to a certain **type** (the method only exists when the generic type is of type `i32` in this case)
- a method that applies to ALL generics

```rust
struct Something<T> {
    thing: T
}

// a method only_for_i32s which, as the name suggests, is only defined
// for instances of Something with i32 parameters
impl Something<i32> {
    fn only_for_i32s(&self) {}
}

// Why does this fail??
impl Something<T> {
    fn generic_function(&self) {}
}
```

The reason it fails is because in this case, `T`, is **undefined**. Think back to the `i32` and note that it was already
defined, but
in the second case, you are basically telling the compiler "look up T and apply this only to them". But T wasn't defined
yet. So,

```rust

impl<T> Something<T> {
    fn generic_function(&self) {}
}
```

and you'll see that this is why we have

```rust
impl<T> Stack<T> {}
```

Discussion sources:

- [Why does Rust require generic type declarations after the "impl" keyword?](https://stackoverflow.com/questions/45473626/why-does-rust-require-generic-type-declarations-after-the-impl-keyword/45473717#45473717)
- [Rust Generic Types in Method definitions](https://dev.to/talzvon/rust-generic-types-in-method-definitions-4iah)

# Lesson 3: fmt::Display and Debug

To debug you'll want to use `println!("{:?}", v)` which will call v's debug. Note: calling `println!("{:#?}", v)`, note
the `#` makes it "pretty-print".

## Debug

Debug can be "imported" easily by just `derive`-ing from it

```rust
#[derive(Debug)]
pub struct Stack<T>
```

but it's only applicable for "simple" types. You might need to implement your own `DEBUG` for custom classes

## Display

```rust
impl<T> fmt::Display for Stack<T> where
    T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack: [")?;
        for el in self.data_vec.iter().rev() {
            write!(f, "{el}, ")?;
        }
        write!(f, "]")
    }
}
```

Some notes about the above:

- Notice how we're doing `impl<T> fmt::Display for Stack<T>`. This means we're implementing the `Display` method for a generic `T`
- Notice the `?` after the `write!`. This is because `write` returns a `Result` and `?` is a quick way of exiting if we hit an `Err`
- The **most** important part of how this works is the 


```rust
where 
    T: Display
```

because that imposes a strict requirement that any type, `T` that is coming in MUST have a `Display` implemented.

# Lesson 4: mods and use

```rust

mod x;

// equivalent to python's import x
use x::y // equivalent to from x import y

```