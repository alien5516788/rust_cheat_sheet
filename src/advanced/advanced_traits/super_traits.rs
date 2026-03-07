// ==========================================
// RUST SUPERTRAITS CHEATSHEET
// ==========================================

/// 1. BASIC SUPERTRAIT DEFINITION
/// A supertrait is a dependency requirement.
/// 'Sub' must implement 'Super' to be valid.
trait Super {
    fn basic_action(&self);
}

// Syntax: trait Sub: Super
trait Sub: Super {
    fn advanced_action(&self);
}

/// 2. MULTIPLE SUPERTRAITS
/// You can require a type to implement multiple traits using the '+' syntax.
trait Reader {
    fn read(&self);
}

trait Writer {
    fn write(&self);
}

// To implement 'SuperEditor', a type MUST implement Reader AND Writer.
trait SuperEditor: Reader + Writer {
    fn atomic_edit(&self) {
        self.read();
        self.write();
        println!("Atomic edit complete.");
    }
}

/// 3. IMPLEMENTATION REQUIREMENTS
/// You cannot implement a subtrait without implementing all its supertraits first.
struct Document;

// Requirement A: Implement Reader
impl Reader for Document {
    fn read(&self) {
        println!("Reading document...");
    }
}

// Requirement B: Implement Writer
impl Writer for Document {
    fn write(&self) {
        println!("Writing document...");
    }
}

// Final Step: Implement the Subtrait
impl SuperEditor for Document {}

/// 4. USAGE IN GENERICS (Trait Bounds)
/// Supertraits allow for cleaner function signatures.
/// Instead of T: Reader + Writer + SuperEditor, just use T: SuperEditor.
fn perform_maintenance<T: SuperEditor>(tool: T) {
    // All methods from Reader, Writer, and SuperEditor are available here.
    tool.read();
    tool.write();
    tool.atomic_edit();
}

/// 5. THE "DYNTRAIT" LIMITATION (Important Note)
/// Rust currently does not allow creating a trait object (dyn)
/// from a trait with multiple supertraits easily if you need
/// to upcast back to the supertraits.
///
/// Example of a valid trait object:
// let editor: &dyn SuperEditor = &Document;

/// 6. COMMON STANDARD LIBRARY EXAMPLES
/// - trait Copy: Clone {}      (To be Copy, you must be Clone)
/// - trait Eq: PartialEq {}    (To be Eq, you must be PartialEq)
/// - trait Ord: PartialOrd + Eq {}

fn main() {
    let doc = Document;
    perform_maintenance(doc);
    println!("Rust supertraits: Checked!");
}
