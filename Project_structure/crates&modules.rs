// Crates ----------------------------------------

/* Crates are the smallest compilation unit in rust.
   Crates can have one to many source files (within the same level
   or within sub folders). */

/* Two types of crates, */
    // Binary crate
    // Library crate

/* Binary crate ---------*/
    /* All the the files belong to a single crate is compiled
       to create the final executable. */

/* Library crate --------*/
    // Used to share amoung other crates as libraries
    // Are compiled into library files '.rlib'





// Modules ---------------------------------------

/* File folder hierarchy and divisions within a file of a crate
   are identified as modules.
   Rust does not consider a source files as a files.
   but as modules */

/* Ways to make modules */
    // By using the 'mod' keyword and curly braces.
    // A single source file is identified as a module.
        // File name will be the module name.
    // A folder with a file named mod.rs is identified as a module.
        // Folder name will be the module name.
        // Content of the module is written inside the 'mod.rs'.
        /* Folderes without mod.rs is not identified as a module
           and hence won't have any content specific to that module. */
