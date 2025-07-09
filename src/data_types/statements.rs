// expression
7 + 8 /* Evaluates to 15 . */ ; // ignore the semicolon
"hello" /* Evaluates to "hello" . */ ; // ignore the semicolon
5.6 /* Evaluates to just 5.6 . */ ; // ignore the semicolon

// statement
7 + 8; /* Evaluates to 15 and terminates itself.
        Semicolon is used to terminate the expression */

// block expression
{} /* A block without any expression evaluates to unit type (explained below).
        Somehow the semicolon is not required here. Don't know why XD !!!! */

{ () }; // Explicitly evaluating to a unit type and terminates

{ 8 + 9 }; // Evaluating to 17 and terminates

{ 8 + 9; } /* Again evaluating to a unit type.
            Because the expression inside the block is terminated. */
