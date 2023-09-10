fn main2() {
    /// In this code we are exploring the slice type.
    /// Slice is, at least as I understand now, a slice of a String.
    /// As of now we are using it to find out why it is better than returning the index of the first word, when the mutable s String can change later, and our index doesn't update with it.
    /// TBD how the immutable reference &str slice reacts to the mutable s being changed.
    let mut s: String = String::from("Hello world!");

    let word: &str = first_word(&s);
    // s.clear();  // compiler will throw error. This is because if we have an immutable reference to something we cannot have a mutable reference to that same thing.
    // The compiler previously did not care that s was mutable from instantiation, because it saw that value did not change.

    // as long as s String is not mofidied, the word &str slice reference into String s is still valid.
    // however, if we use s.clear() before calling println!(), the compiler will catch this error for us.
    // It makes sure that the immutable reference into String remains valid, and recognizes the error if s is modified before word goes out of scope.
    // This is much better way than just returning the index, because the compiler will catch this error for us.
    // If we just used the index, s.clear() followed by printing a String slice manually using the index, a runtime error will occur.
    // This is because the index and s String are unrelated until the index is used on String.
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    // &str return type is an immutable reference to a slice type.

    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // the .. operator without any start or stop means the entire String as a slice.
    &s[..]
}

fn string_literals() {
    let s: &str = "Hello world";
    // s is actually a slice to a string literal stored inside the binary.
    // it is not a pointer to a value in memory. It points to a specific line in the binary.
    // this is why string literals are an immutable reference.

}

fn main() {
    let my_string: String = String::from("Hello world!");

    let word: &str = better_first_word(&my_string[0..6]);
    let word2: &str = better_first_word(&my_string[6..]);
    let word3: &str = better_first_word(&my_string);
    // all of the above are valid because String my_string is converted to a string slice by taking a reference to that value, whether or not we decided to take a subset of the (now) slice of not.

    println!("word: {}", word);
    println!("word2: {}", word2);
    println!("word3: {}", word3);

    // ok, now instantiating a string literal (which is a slice), and using it in the better_first_word() function.
    let word4: &str = "Hello world!";

    // when taking a slice of a string literal, we still need to use the & immutable reference marker.
    let word5: &str = better_first_word(&word4[0..]);
    let word6: &str = better_first_word(word4);
    // ^ here, we don't need to use the immutable reference marker '&', because we are not taking a subset of the string literal. Remember, a string literal is already a slice by definition, because it is an immutable value in the binary.
}

fn better_first_word(s: &str) -> &str {
    // this function is a better implementation of first_word().
    // why?
    // because the function signature uses the slice &str parameter instead of a String pointer.
    // This allows both immutable str slices to be passed in, OR a String that has been converted to a str slice OR a reference to the String.
    // this takes advantage of a feature called deref coercions, a feature studied towards the end of the book.
    // This makes the API more general and useful without sacrificing functionality.

    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slice_types() {

    // an array of i32 values
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // this is an array slice
    let slice: &[i32] = &arr[2..4];
}