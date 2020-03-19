// ownership is a central feature of rust
// rust manages it's memory through "ownership"
// ownership is based on rules that the compiler checks at compile time

// The Stack and the Heap
/// Both are parts of memory that are available at runtime
/// The stack stores values in the order it gets them
/// The stack retrieves data in the opposite order
/// The stack's method of data storage is known as "last in, first out"
/// Adding data to the stack is called "pushing onto the stack"
/// Retrieving data from the stack is called "popping off the stack"
/// All data on the stack must have a known, fixed size
/// Data with unknown/changing size at compile time must be stored on the heap
/// When you put data on the heap you request a certain amount of space
/// The OS finds an empty spot in the heap that is big enough then:
///  Marks it as being in use
///  Returns a pointer (address to that spot)
/// The heap's method of data storage is called "allocating on the heap"
/// Pushing values onto the stack is not considered allocating because the pointer is a known, fixed size
/// You can store the pointer on the stack but if you want the data you must follow the pointer
/// Pushing to the stack is faster than allocating on the heap
/// Access data in the heap is slower than accessing data on the stack
/// When functions are called the args are pushed onto the stack
/// Function local variables are also pushed onto the stack
/// When the function is finished those values are popped off the stack
/// Ownership allows you to minimize the amount of duplicate data on the heap
/// Ownership allows you to clean up unused data on the heap (avoids running out of space)

// Ownership Rules
/// Each value in Rust has a variable that's called its owner
/// There can only be one owner at a time
/// When the owner goes out of scope, the value will be dropped

// Variable Scope
/// Range within a program for which an item is valid
///

// String Type
/// str is a string literal and is a fixed sized
/// String is allocated on the heap and is able to store an amount of text that is unknown

// s is not valid here because it’s not yet declared
fn main() {
    let s = "hello"; // s is valid from this point forward
    let mut s = String::from(s); // converts string literal to a mutable string type by calling for memory on the heap through the os
                                 // :: is an operator that allows us to namespace this particular "from" function under the string type rather than using some sort of name like string_from
                                 // This type of string can be mutated
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // doing stuff with s while it's valid

    let x = 5; // both x and y are pushed onto the stack because they are integer values with a simple fixed known size (5: i32)
    let y = x; // binds 5 to x and copies value of x and binds it to y

    // String is made up of three parts: a pointer to the memory that holds the content of a string, a length and a capacity
    // this group of String data is pushed to the stack but the memory that holds the actual contents is stored on the heap
    // pointer stored on the stack for the string points to the content in the heap
    // the length is how much memory (in bytes) the contents of the String is currently using
    // the capacity is the total amount of memory (in bytes) that the String has received from the OS
    let s1 = String::from("hello"); // requests allocation on the heap for a new String
    let s2 = s1; // Content is not copied only the data pointing to the same content is copied
                 // Pointer, length and capacity data from s1 is copied to a new variable called s2 and stores the same data as s1 on the stack
                 // if s1 and s2 try to go out of memory at the same time it will try to free the same data and will cause a double free error
                 // freeing memory twice can lead to memory corruption which can potentially lead to a security vulnerability
                 // to ensure memory safety Rust considers s1 no longer valid
                 // this isn't a shallow copy because the first variable is no longer valid (it's called a move)
                 // it is to be said that s1 was moved to s2
                 // since s2 is the only valid variable with the same data it is the only String that rust frees at the end of the scope
    let s3 = s2.clone(); // if we want a deep copy (more resource expensive because it is copying contents and requesting a new spot on the stack) we can call .clone() method
    println!("{} {} {} {}", s2, s3, x, y); // s1 is the only one that wont work because it was "moved"
                                           // x works because integers with a fixed value and size (i32) are stored on the stack and deep copies are quick to make (doesn't invalidate x and both are dropped at the end of the scope)
    take_ownership(s3); // s3's value moves into the function and is no longer valid here
                        // println!("{}", s3); // this is invalid because s3 was just moved
    make_copy(x); // x's value moves into the function but i32 is Copy so it's okay to still use x afterward
                  // println!("{}", x); // this is valid because i32 is Copy and isn't moved
    let s3 = take_and_give_back(s2); // s2 is moved into the function and the return value is moved into s3
    println!("{}", s3);
    let (s2, s3) = return_multiple_values(s3); // ownership of s3 is sent to the function and the function returns a tuple with multiple Strings that are stored on the new into new variables
    println!("{} {}", s2, s3); // both s2 and s3 are valid now
                               // because of this when s2 goes out of scope nothing happens (it was moved) but now s3 is dropped at the end of this scope (main owns it still)
} // now s isn't dropped because it was moved but x is still dropped
  // this scope is now over and s is no longer valid and the memory for our String type is returned to the OS
  // we know the contents of string literals at compile time so the text is hard coded directly into the final executable
  // string literals are immutable
  // String type supports mutability because it is allocated on the heap
  // this means that the memory must be requested from the operating system at runtime
  // we need a way of returning this memory to the OS when we're done with the String
  // Rust automatically returns memory when a variable goes out of scope
  // Rust automatically called a function called drop at the end of a block

// if a type has the Copy trait then an older variable is still usable after assignment
// Rust won't allow type if the type has implemented the drop trait
// any group of simple scalar values can be Copy and nothing that requires allocation or is some form of a resource is Copy

// Types that are Copy:
// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

fn take_ownership(a_string: String) {
    // a_string comes into scope
    println!("{}", a_string);
} // a_string goes out of scope and drop is called
  // memory is then freed

fn make_copy(a_int: i32) {
    // a_int comes into scope
    println!("{}", a_int);
} // a_int goes out of scope and nothing happens because ownership is restored (Copy type)

fn take_and_give_back(a_string: String) -> String {
    a_string // a_string is returned and moved out to the calling function
}
// these are examples of ownership being transferred between functions

// when a variable that includes data on the heap goes out of scope
// the value will be cleaned up by drop
// unless the data has been moved to be owned by another variable

fn return_multiple_values(a_string: String) -> (String, String) {
    let another_string = a_string.clone(); // deep cloned into a new string variable
    (another_string, a_string) // returns a tuple with the new cloned string and the old string
                               // ownership of both the new and old string are returned to the calling function
}
