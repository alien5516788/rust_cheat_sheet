fn variables() {
    // `let name: DataType = value;`
    // By default all varibles are immutable
    
    // Normal variable
    let var : i32;
        var = 4;
    
    let var = 4;
    
    let var : i32 = 4;
    
    // Mutable variable
    let mut var : i32;
        var = 4;
    
    let mut var = 4;
    
    let mut var : i32 = 4;
    
    // Constant
    const VAR : i32 = 4;
    
    // Empty variable
    // `_` Just takes out a value and discard it
    let _ = 34;
    _ = 34;

}

fn statement_expression() {
    
    // expression
    7 + 8   /* Evaluates to 15 . */ ; // ignore the semicolon
    "hello" /* Evaluates to "hello" . */ ; // ignore the semicolon
    5.6     /* Evaluates to just 5.6 . */ ; // ignore the semicolon
    
    // statement
    // Semicolon is used to terminate the expression 
    7 + 8; // Evaluates to 15 and terminates itself
           
    
    // block expression
    {}          // A block without any expression evaluates to unit type (explained below).
                // Somehow the semicolon is not required here. Don't know why XD !!!! */
    
    { () };     // Explicitly evaluates to unit type and terminates
    
    { 8 + 9 }; // Evaluating to 17 and terminates
    
    { 8 + 9; } // Again evaluates to unit type
               // Because the expression inside the block is terminated

}
