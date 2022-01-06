//! Shared reference: &
//! Mutable reference: &mut
//? A reference cannot outlive its referent: we can think of
//? referent as a scope. The reference cannot outlive the
//? existence of thy scope.
//? A mutable reference cannot be aliased: ?

//! Aliasing:
//? Variables and pointers alias if they refer to OVERLAPPING
//? REGIONS OF MEMORY.
//? &mut is not allowed to be ALIASED.
//? <https://doc.rust-lang.org/nomicon/aliasing.html>
//* You can keep values in registers (eliminating a read), knowing
//* that no pointers has access to the value's memory.
//* The key thing to remember about alias analysis is that writes
//* are the primary hazard for optimizations.
//? ALIAS does not matter if writes to memory are not happening.
// TODO: is it possible to keep track of values in registers?
// TODO: Is that the reason ed25519-java was implemented that way?.

//! Lifetimes:
//? Lifetimes coincide with scope.
//? Lifetimes are denoted with an apostrophe: 'a, 'static.
//* let statements introduces a scope (Ex.: let a = b).
//* The lifetime (sometimes called a borrow) is alive from the
//* place it is created to its last use.

//! Lifetime elision (lifetimes in input and output).

//* Example 1
pub fn compute(input: &u32, output: &mut u32) {
    let inp: &u32 = input;
    let out: &mut u32 = output;
    if *inp > 10 {
        *out = 1;
    }
    if *inp > 5 {
        *out *= 2;
    }
}

//* Example 2: Problem: we have a live shared reference x to
//* a descendant of data when we try to take a mutable
//* reference to data to push.
//? Subtle and interesting example: if we swap print and push,
//? it works, the reason is 'x' is no longer needed. Although
//? this works, 'x' exists till the end of the scope.
//? However if the value has a destructor, running the destructor
//? is considered a use that is run at the end of the scope,
//? therefore this will not compile.
fn lifetimes_interesting() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    data.push(4);
    // println!("{}", x);
}

//* Example 2: This works.
fn lifetimes_interesting2() {
    let mut data = vec![1, 2, 3];
    // This mut allows us to change where the reference points to
    let mut x = &data[0];
    // Last use of this borrow
    println!("{}", x);
    data.push(4);
    // We start a new borrow here
    x = &data[3];
    println!("{}", x);
}

//* Example 1: Optimized way is to keep input in a register.
fn compute(input: &u32, output: &mut u32) {
    // keep *input in a register
    let cached_input = *input;
    if cached_input > 10 {
        // x > 10 implies x > 5, so double and exit immediately
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}

//* Example 3: Where clause; closure.alloc.
pub fn call_with_one<F>(some_closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    some_closure(1)
}
