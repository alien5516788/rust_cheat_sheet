pub mod data_types;
pub mod flow_control;
pub mod miscellaneous;
pub mod modularity;

fn main() {
    fn gneric_structs() {
        // Generic struct
        struct Point<T, U> {
            x: T,
            y: U,
        }

        // Generic struct with a const parameter
        struct List<T, const U: usize> {
            t: T,
            l: [i32; U],
        }

        // Generic struct with default type parameter
        struct Item<T, U = f32> {
            t: T,
            l: U,
        }

        let p = Point { x: 1, y: 2.4 };
        let l = List {
            t: "Hello",
            l: [1, 2, 3],
        };
        let i = Item {
            t: "World",
            l: 3.14,
        };
    }
}
