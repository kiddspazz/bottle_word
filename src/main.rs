fn main() {
    for x in (1..=99).rev() {
        let bottle_word = if x > 1 {"bottles"} else {"bottle"};
        let next_bottle_word = if x > 2 {"bottles"} else {"bottle"};

        println!(
            "{number_of_bottles} {noun} of beer on the wall, {number_of_bottles} {noun} of beer.", 
            number_of_bottles = x, noun = bottle_word
        );

        if x > 1 {
            println!(
                "Take one down and pass it around, {number_of_bottles} {noun} of beer on the wall.",
                number_of_bottles = x - 1, noun = next_bottle_word
            );
        } else {
            println!("Take it down and pass it around, no more bottles of beer on the wall.");
        }
        println!("\n");
    }
    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottle_word of beer on the wall.");
}
