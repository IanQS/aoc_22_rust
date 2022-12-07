# Focus:

`traits`, `impl` and `struct`



## Structs

- a structure that can hold information

### Structs: Quiz

- [Official Docs](https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html) + 

```rust
// What's the difference between all of the following?

pub struct PubStruct<T> {  // case 1
    pub field: T
}

pub struct PubOuter<T>{ // case 2
    field: T
}

struct PubInner <T>{ // case 3
    pub field: T
}

struct PubNone <T>{ // case 4
    field: T
}
```


### Structs: Solution

#### PubStruct

The first method is public, so we can construct directly with the value names e.g

```rust
let a: PubStruct = PubStruct{field: -1};
```

#### PubOuter
whereas in the second one, we cannot construct the struct directly (bc of private field) 

```rust

let b: PubOuter = PubOuter{field: 1} //results in error
```

How do we fix this? 

```rust
impl<T> PubOuter<T>{
    pub fn new(some_entry: T) -> Self{
        PubOuter {field: some_entry}
    }
}

let b: PubOuter = PubOuter::new("Some value");
```

#### PubInner

- realistically, nothing special here. You can't access the fields as the outer scope is private. BUT could be used to indicate.... something(?)

#### PubNone

- can't be seen outside the cargo or whatever this is sitting in.

---