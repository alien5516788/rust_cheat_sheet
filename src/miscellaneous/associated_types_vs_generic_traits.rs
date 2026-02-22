// ========================================================== TODO
// ASSOCIATED TYPE VS GENERIC TRAIT PATTERN
// ==========================================================

// Comparing two ways of encoding type abstraction:

// 1. Generic trait: the type is a generic parameter
trait TraitA<T> {
    fn process(&self, value: T);
}

// 2. Associated type: the type is tied to the trait impl
trait TraitB {
    type Item;
    fn process(&self, value: Self::Item);
}

// Associated types are often cleaner when the type is intrinsic to the trait,
// while generics are better when you want more flexibility or multiple type parameters.
