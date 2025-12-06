# tagged_delegate

Automatically generate delegation macros for enums without requiring traits.

## Overview

`tagged_delegate` generates delegation macros when applied to enums. Unlike `enum_dispatch` which requires annotating traits, `tagged_delegate`:
- Works without traits entirely
- Can delegate to methods on types you don't own
- Requires the enum to be wrapped in a struct

## Example
```
use tagged_delegate::tagged_delegate;

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

    fn zero(&mut self) {
        mut_delegate!(self, |s| s.set_position((0.0, 0.0)));
    }
}

struct Circle {
    radius: f64,
    position: (f64, f64),
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn set_position(&mut self, new_position: (f64, f64)) {
        self.position = new_position;
    }
}

struct Rectangle {
    width: f64,
    height: f64,
    position: (f64, f64),
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn set_position(&mut self, new_position: (f64, f64)) {
        self.position = new_position;
    }
}
```

## License

Apache-2.0
