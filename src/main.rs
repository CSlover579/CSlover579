// fn main() {
//   let mut s = String::from("hello");
//   s.push_str(", world!"); // push_str() appends a literal to a String
//   println!("{}", s); // This will print `hello, world!`
// }
// In other words, there are two important points in time here:
// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.
// At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the String type.

// fn main() {
//     let s = String::from("hello");  // s comes into scope
//     takes_ownership(s);             // s's value moves into the function... and so is no longer valid here
//     let x = 5;                      // x comes into scope
//     makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.




// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1
//     let s2 = String::from("hello");     // s2 comes into scope
//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.
// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it
//     let some_string = String::from("yours"); // some_string comes into scope
//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }
// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope
//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of '{}' is {}.", s2, len);
// }
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//     (s, length)
// }

// fn main() {
//     let mut s1 = String::from("Hello");
//     calculate_length(&mut s1);
//     print!("{}", s1);
// }
// fn calculate_length(s1: &mut String){
//     s1.push_str(" world");
// }
// First, notice that all the tuple code in the variable declaration and the function return value is gone.
// Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.
// These ampersands(&) represent references, and they allow you to refer to some value without taking ownership of it.
// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.

// fn main(){
//   let mut s = String::from("hello");
//   let r1 = &mut s;
//   let r2 = &mut s;
//   println!("{}, {}", r1, r2);
// }
// // $ cargo run
// //   Compiling ownership v0.1.0 (file:///projects/ownership)
// // error[E0499]: cannot borrow `s` as mutable more than once at a time
// //  --> src/main.rs:5:14
// //  |
// 4 |     let r1 = &mut s;
//   |              ------ first mutable borrow occurs here
// 5 |     let r2 = &mut s;
//   |              ^^^^^^ second mutable borrow occurs here
// 6 |
// 7 |     println!("{}, {}", r1, r2);
//   |                        -- first borrow later used here
// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `ownership` (bin "ownership") due to 1 previous error
// This error says that this code is invalid because we cannot borrow s as mutable more than once at a time.
// The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.


// fn main(){
//   let mut s = String::from("hello");
//   {
//       let r1 = &mut s;
//   } // r1 goes out of scope here, so we can make a new reference with no problems.
//   let r2 = &mut s;
// }

// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }
// $ cargo run
//    Compiling ownership v0.1.0 (file:///projects/ownership)
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//  --> src/main.rs:6:14
//   |
// 4 |     let r1 = &s; // no problem
//   |              -- immutable borrow occurs here
// 5 |     let r2 = &s; // no problem
// 6 |     let r3 = &mut s; // BIG PROBLEM
//   |              ^^^^^^ mutable borrow occurs here
// 7 |
// 8 |     println!("{}, {}, and {}", r1, r2, r3);
//   |                                -- immutable borrow later used here

// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `ownership` (bin "ownership") due to 1 previous error
// Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
// Users of an immutable reference don’t expect the value to suddenly change out from under them!
// However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
// For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }
// The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created.
// These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.

// fn main() {
//     let reference_to_nothing = no_dangle();
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// $ cargo run
//    Compiling ownership v0.1.0 (file:///projects/ownership)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:5:16
//   |
// 5 | fn dangle() -> &String {
//   |                ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
//   |
// 5 | fn dangle() -> &'static String {
//   |                 +++++++
// help: instead, you are more likely to want to return an owned value
//   |
// 5 - fn dangle() -> &String {
// 5 + fn dangle() -> String {
//   |

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `ownership` (bin "ownership") due to 1 previous error
// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it.
// That means this reference would be pointing to an invalid String.
// That's no good! Rust won't let us do this.
// Solution:
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// Here’s a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.
// Next, we create an iterator over the array of bytes using the iter method:
// For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
// The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.
// Because the enumerate method returns a tuple, we can use patterns to destructure that tuple.
// In the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple.
// Because we get a reference to the element from .iter().enumerate(), we use & in the pattern
// Inside the for loop, we search for the byte that represents the space by using the byte literal syntax.
// We now have a way to find out the index of the end of the first word in the string, but there’s a problem.
// We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String.
// In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.
fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s); // word will get the value 5
  s.clear(); // this empties the String, making it equal to ""
  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
}
// This program compiles without any errors and would also do so if we used word after calling s.clear().
// Because word isn’t connected to the state of s at all, word still contains the value 5.
// We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.
// Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
// Managing these indices is even more brittle if we write a second_word function. Its signature would have to look like this:
// fn second_word(s: &String) -> (usize, usize) {
// Now we’re tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all.
// We have three unrelated variables floating around that need to be kept in sync.
