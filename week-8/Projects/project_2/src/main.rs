use std::io;

fn main() {
    // Get the number of candidates
    let mut total_candidates_input = String::new();

    println!("Enter the number of candidates:");
    io::stdin()
        .read_line(&mut total_candidates_input)
        .expect("Failed to read input");

    let total_candidates: usize = total_candidates_input
        .trim()
        .parse()
        .expect("Please enter a valid whole number");

    // Store each candidate's name and experience in a vector of tuples
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Collect information about each candidate
    for i in 0..total_candidates {
        println!("\nEnter candidate {}'s name:", i + 1);

        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read name");

        let name = name_input.trim();

        println!("Enter years of programming experience:");

        let mut experience_input = String::new();
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read experience");

        let experience: u32 = experience_input
            .trim()
            .parse()
            .expect("Please enter a valid whole number");

        // Add the candidate's name and experience to the vector
        candidates.push((name.to_string(), experience));
    }

    // Check whether any candidates were entered
    if candidates.len() == 0 {
        println!("No candidates were entered.");
    } else {
        // Assume the first candidate has the highest experience
        let mut highest = candidates[0].1;

        // Compare the other candidates' experience with the highest so far
        for i in 1..candidates.len() {
            if candidates[i].1 > highest {
                highest = candidates[i].1;
            }
        }

        // Display every candidate who has the highest experience
        println!("\nCandidate(s) with the highest programming experience:");

        for i in 0..candidates.len() {
            if candidates[i].1 == highest {
                println!("Name: {}", candidates[i].0);
                println!("Years of experience: {}", candidates[i].1);
            }
        }
        println!("Total candidates interviewed: {}", candidates.len());
    }
}
