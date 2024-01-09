/*A Marco Polo Game
If the name Marco is given, the program will respond with Polo
Otherwise program will respond with "what's your name?".
*/

// create a pub marco function in rust
pub fn marco_polo(name: &str) -> String {
    // check if the input string matches the word 'Marco'
    let match_marco = name.eq("Marco");
    // return an appropriate response based on whether or not the names matched 
    match match_marco {
        true => format!("Polo {}!", name),   // if they did match, return
        // "Polo [inserted name]!"
        false => "What's your name?".to_string() // otherwise, return what
        // "What's Your Name?"
    }
}
