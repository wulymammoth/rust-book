/// COMMON BEHAVIOR THAT IS EXTENSIBLE
///
/// - we can't know all the types that other devs want to create at the outset
/// - we just declare an interface (behavior) that we want comformity with
///   * similar to "duck-typing" where we are more concerned only with the messages a value
///   responds to rather than the value's concrete type
///
/// * SCENARIO
///   we want to be able to iterate across a vector that takes a 'trait' object
///   - a trait object can be thought of as a concrete implementation of a trait or an object
///   (instance) that exhibits a particular trait (behavior)
///   - they are a substitute for generic or concrete types
///   * note that data cannot be added to a trait object (major difference compared to OO in other
///   languages)
pub trait Draw {
    fn draw(&self);
}

// - struct definition, `Screen`, holding a vector named `components`
// * the `components` vector is of type `Box<dyn Draw>` (trait object)
//   * IT IS A STAND-IN/PLACEHOLDER FOR ANY TYPE INSIDE A `Box` THAT IMPLEMENTS THE `Draw` TRAIT
// * NOTE: the alternative that this is different from is defining a struct that uses a generic
// type parameter with trait bounds, where we need to know all the types up front and can only be
// substituted with one concrete type at a time whereas we're trying to support MULTIPLE types that
// exhibit the same behavior
// * NOTE: there is a runtime cost associated with trait objects and that's *dynamic dispatch*
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // invoking the draw method on a component that's of a `Draw` trait object
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// the alternative with generic type parameters (non-trait object)
//
// * this is restrictive
//   - must be a `Screen` instance
//   - components vector must contain elements that are all of the same type, e.g., `Button`
// * NOTE: CAVEAT is when we know that we'll only ever have homogenous collections, using generics
// and trait bounds is prefereable because the definitions will be **monomorphized** at compile
// time to use the concrete types // in other words: removes overhead of dynamic dispatch AKA
// static dispatch
pub struct ScreenAlt<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenAlt<T>
where T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// defining some types that implement the `Draw` trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // not implemented (for demo only)
    }
}
