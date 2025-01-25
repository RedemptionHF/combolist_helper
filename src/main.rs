use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use indicatif::{ProgressBar, ProgressStyle};

fn generate_email() -> String {
    let domains = vec!["gmail.com", "yahoo.com", "outlook.com", "hotmail.com", "aol.com"];
    let username: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let domain = domains[thread_rng().gen_range(0..domains.len())];
    format!("{}@{}", username, domain)
}

fn generate_username() -> String {
    let username: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}", username)
}

fn generate_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

fn prompt_memory_mode() -> io::Result<bool> {
    println!("Select memory mode:");
    println!("[1] Higher memory usage, but only writes to disk once.");
    println!("[2] Better performance, but constantly writes to the disk.");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim() {
        "1" => Ok(true),  // High-memory mode
        "2" => Ok(false), // Low-memory mode
        _ => {
            println!("Invalid choice. Defaulting to better performance.");
            Ok(false)
        }
    }
}

fn convert_email_to_user() -> io::Result<()> {
    println!("Enter the input file name (e.g., input.txt):");
    let mut input_file_name = String::new();
    io::stdin().read_line(&mut input_file_name)?;
    let input_file_name = input_file_name.trim();

    println!("Enter the output file name (e.g., output.txt):");
    let mut output_file_name = String::new();
    io::stdin().read_line(&mut output_file_name)?;
    let output_file_name = output_file_name.trim();

    let input_file = File::open(input_file_name)?;
    let reader = BufReader::new(input_file);
    let total_lines = reader.lines().count();

    let input_file = File::open(input_file_name)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(output_file_name)?;

    let progress_bar = ProgressBar::new(total_lines as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec} lines/sec")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Processing data");

    for line in reader.lines() {
        let line = line?;
        if let Some((email, password)) = line.split_once(':') {
            if let Some(username) = email.split('@').next() {
                writeln!(output_file, "{}:{}", username, password)?;
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Data manipulation complete!");
    println!("Output written to {}", output_file_name);

    Ok(())
}

fn generate_fake_combolist() -> io::Result<()> {
    let high_memory = prompt_memory_mode()?;

    println!("Enter the name of the file to save generated data (e.g., fake_data.txt):");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();

    println!("How many email:password pairs would you like to generate?");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = input.trim().parse().unwrap_or(100);

    let mut file = BufWriter::new(File::create(file_name)?);
    let progress_bar = ProgressBar::new(count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec} lines/sec")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Generating data");

    // Pre-generate the data into a vector to optimize I/O operations

    if high_memory {
        let mut combo_list = Vec::with_capacity(count);
        for _ in 0..count {
            combo_list.push(format!("{}:{}", generate_email(), generate_password()));
            progress_bar.inc(1);
        }
        for combo in combo_list {
            writeln!(file, "{}", combo)?;
        }
    } else {
        for _ in 0..count {
            writeln!(file, "{}:{}", generate_email(), generate_password())?;
            progress_bar.inc(1);
        }
    }

    progress_bar.finish_with_message("Data generation complete!");
    println!("Generated {} email:password pairs and saved to {}", count, file_name);

    Ok(())
}

fn generate_fake_data() -> io::Result<()> {
    let high_memory = prompt_memory_mode()?;
    let username_file = "username_data.txt";
    let email_file = "email_data.txt";
    let password_file = "password_data.txt";

    println!("How many usernames, emails, and passwords would you like to generate?");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = input.trim().parse().unwrap_or(100);

    let mut username_save_file = BufWriter::new(File::create(username_file)?);
    let mut email_save_file = BufWriter::new(File::create(email_file)?);
    let mut password_save_file = BufWriter::new(File::create(password_file)?);

    let progress_bar = ProgressBar::new(count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec} lines/sec")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Generating data");

    let mut usernames = Vec::with_capacity(count);
    let mut emails = Vec::with_capacity(count);
    let mut passwords = Vec::with_capacity(count);

    // Pre-generate the data and collect it into vectors
    if high_memory {
        for _ in 0..count {
            let email = generate_email();
            let username = generate_username();
            let password = generate_password();
            usernames.push(username);
            emails.push(email);
            passwords.push(password);
            progress_bar.inc(1);
        }

        // Write all the data at once to the files using the buffered writers
        for username in usernames {
            writeln!(username_save_file, "{}", username)?;
        }

        for email in emails {
            writeln!(email_save_file, "{}", email)?;
        }

        for password in passwords {
            writeln!(password_save_file, "{}", password)?;
        }
    } else {
        for _ in 0..count {
            let email = generate_email();
            let username = generate_username();
            let password = generate_password();
            writeln!(username_save_file, "{}", username)?;
            writeln!(email_save_file, "{}", email)?;
            writeln!(password_save_file, "{}", password)?;
            progress_bar.inc(1);
        }
    }
    progress_bar.finish_with_message("Data generation complete!");
    println!("Generated {} usernames, emails, and passwords.", count);

    Ok(())
}

fn create_combolist() -> io::Result<()> {
    println!("Enter the file with usernames/emails (e.g., usernames.txt):");
    let mut user_file_name = String::new();
    io::stdin().read_line(&mut user_file_name)?;
    let user_file_name = user_file_name.trim();

    println!("Enter the file with passwords (e.g., passwords.txt):");
    let mut pass_file_name = String::new();
    io::stdin().read_line(&mut pass_file_name)?;
    let pass_file_name = pass_file_name.trim();

    println!("Enter the output file name for the combo list (e.g., combo.txt):");
    let mut output_file_name = String::new();
    io::stdin().read_line(&mut output_file_name)?;
    let output_file_name = output_file_name.trim();

    let user_file = File::open(user_file_name)?;
    let pass_file = File::open(pass_file_name)?;
    let mut output_file = File::create(output_file_name)?;

    let user_reader = BufReader::new(user_file);
    let pass_reader = BufReader::new(pass_file);

    let users: Vec<String> = user_reader.lines().filter_map(|line| line.ok()).collect();
    let passwords: Vec<String> = pass_reader.lines().filter_map(|line| line.ok()).collect();

    let total_combos = users.len();
    let progress_bar = ProgressBar::new(total_combos as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec} combos/sec")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Creating combo list");

    // Pair each user with a password sequentially, reusing passwords if necessary
    for (i, user) in users.iter().enumerate() {
        let password = &passwords[i % passwords.len()]; // Loop over passwords if we run out
        writeln!(output_file, "{}:{}", user, password)?;
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Combo list creation complete!");
    println!("Combo list saved to {}", output_file_name);

    Ok(())
}

fn main() {
    loop {
        println!("\nSelect an option:");
        println!("[1] Email:password to user:password converter");
        println!("[2] Create combo list (username/email + passwords)");
        println!("[90] Generate fake email:password data");
        println!("[91] Generate fake usernames, emails and passwords");
        println!("[99] Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                if let Err(err) = convert_email_to_user() {
                    eprintln!("Error: {}", err);
                }
            }
            "2" => {
                if let Err(err) = create_combolist() {
                    eprintln!("Error: {}", err);
                }
            }
            "90" => {
                if let Err(err) = generate_fake_combolist() {
                    eprintln!("Error: {}", err);
                }
            }
            "91" => {
                if let Err(err) = generate_fake_data() {
                    eprintln!("Error: {}", err);
                }
            }
            "99" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}