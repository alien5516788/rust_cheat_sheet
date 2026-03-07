fn default_type_for_generics() {
    // Default type for generic parameter are not supported for functions

    struct Item<T, U = f32> {
        t: T,
        l: U,
    }

    enum Output3<T, U = f32> {
        Value1(T),
        Value2(U),
    }

    trait MyTrait<T = i32> {
        fn do_something(&self, value: T);
    }
}
