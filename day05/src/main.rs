fn main() {
    let input = include_str!("../input.txt").trim();

    let polymer = reduce_polymer(&input, None);

    println!("The length of the polymer is {}", polymer.len());

    // Try removing all possible units and determine which polymer is the shortest
    let improved_polymer = (b'a'..=b'z')
        .map(|c| reduce_polymer(&input, Some(c as char)))
        .min_by_key(|p| p.len())
        .expect("Could not find improved polymer.");

    println!("The length of the polymer is {}", improved_polymer.len());
}

fn reduce_polymer(input: &str, ignore: Option<char>) -> Vec<char> {
    // This approach uses a stack to keep track of when a reaction occurs.
    // When iterating through each character, we check the top of the stack to see
    // if a reaction occurs, if so the stack is popped, otherwise the unit is pushed
    // on the stack.
    //
    // For part two, we need to figure out the shortest polymer that can be created
    // by removing a single unit, so this function optionally allows an optionally
    // specified unit to be ignored.
    input.chars().fold(Vec::new(), |mut current_polymer, curr| {
        if ignore
            .map(|ignore| ignore.eq_ignore_ascii_case(&curr))
            .unwrap_or(false)
        {
            return current_polymer;
        }

        if current_polymer
            .last()
            .map(|prev| reaction_occurs(*prev, curr))
            .unwrap_or(false)
        {
            current_polymer.pop();
        } else {
            current_polymer.push(curr);
        }

        current_polymer
    })
}

fn reaction_occurs(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}
