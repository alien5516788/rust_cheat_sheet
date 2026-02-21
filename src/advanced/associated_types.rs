fn associated_types() {
    
    // Associated types are only supported in traits
    trait Container {
        type Item;

        fn get(&self) -> Self::Item;
    }

    struct Boxed<T> {
        value: T,
    }

    impl<T: Copy> Container for Boxed<T> {
        type Item = T;

        fn get(&self) -> Self::Item {
            self.value
        }
    }

    let b = Boxed { value: 10 };
    let _v = b.get();
}