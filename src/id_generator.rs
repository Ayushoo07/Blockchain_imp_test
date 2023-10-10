
pub fn generate_id()->String
{
let file_path = "input.txt";
let mut input = String::new();

// Attempt to open the file for reading
let mut file = match File::open(&file_path) {
    Ok(file) => file,
    Err(_) => {
        // If the file doesn't exist or there's an error, create a new file.
        let mut new_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file_path)
            .expect("Failed to create file");
        // Initialize the file with a default value
        new_file
            .write_all(b"abcdefghijklmnopqrstuvwxyz")
            .expect("Failed to write to file");
        new_file
    }
};
file.read_to_string(&mut input)
    .expect("Failed to read file");
file.read_to_string(&mut input)
    .expect("Failed to read file");
let mut chars: Vec<char> = input.chars().collect();
chars.sort();
chars.next_permutation();
    let latest =chars.iter().collect();
    let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Truncate the file (remove existing content)
            .open(&file_path)
            .expect("Failed to open file");

        // Write the new content to the file
        file.write_all(latest.as_bytes())
            .expect("Failed to write to file");
}