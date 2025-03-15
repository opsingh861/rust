fn main() {
    // heap and stacks
    let x: i32 = 7; // this will go to stack, as at the compile time we know the size so need to increase the size in near future
    // stack is fast, whenever it will go out of scope it will be deleted.

    let greeting = String::from("Hey there"); // In case of string we are not sure of size so it will be stored in the heap and the pointer will be stored in the stack
    // heap is slow, but we can store large data in it.
    // stack will store the pointer to the heap memory, length and capacity of the string.
    print!(
        "Capacity: {}   Length: {}   Pointer: {:p}\n",
        greeting.capacity(),
        greeting.len(),
        greeting.as_ptr()
    );
}
