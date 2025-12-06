# tagged_delegate

Automatically generate delegation macros for enums without requiring traits.

## Overview

`tagged_delegate` generates delegation macros when applied to enums. Unlike `enum_dispatch` which requires annotating traits, `tagged_delegate`:
- Works without traits entirely
- Can delegate to methods on types you don't own
- Requires the enum to be wrapped in a struct

## Example
```
// You can also use #[tagged_delegate(shape_delegate)], the struct field and generated macro will have that name
#[tagged_delegate]
enum ShapeDelegate {
    Circle(Circle),
    Rectangle(Rectangle),
}

struct Shape {
    delegate: ShapeDelegate
}

impl Shape {
    fn area(&self) -> f64 {
        delegate!(self, |s| s.area())
    }

    fn mutable_thing(&mut self) -> f64 {
        mut_delegate!(self, |s| ... )
    }
}
```

## License

Apache-2.0
