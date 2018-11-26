use std::io::{prelude::*, stdin, stdout};

pub struct UI { }

impl UI {
    pub fn print_introduction() {
        println!();
        println!(">---------------------------------------------------------------v");
        println!("                                                                |");
        println!("    Leaps and Bounds: A tool to find superpermutation bounds    |");
        println!("         Written by Chris Patuzzo in 2018, MIT License          |");
        println!("                                                                |");
        println!("v---------------------------------------------------------------<");
        println!("|");
        println!("|");
        println!("> What is it? --------------------------------------------------v");
        println!("                                                                |");
        println!("  This tool tries to find the maximum number of permutations    |");
        println!("  that can fit into a string that 'wastes' N symbols. A symbol  |");
        println!("  is said to be 'wasted' if it does not add a new permutation.  |");
        println!("                                                                |");
        println!("  For example, the string '1234123421' wastes two symbols.      |");
        println!("                                  ^^                            |");
        println!("                               these two                        |");
        println!("                                                                |");
        println!("  This is because '1234' has already appeared and '2342' is     |");
        println!("  not a valid permutation because the '2' is repeated.          |");
        println!("                                                                |");
        println!("  Without wasting any symbols, the best we can do is fit four   |");
        println!("  permutations in a string (e.g. 1234123). What about if        |");
        println!("  we're allowed to waste one symbol? What about two? Three?     |");
        println!("                                                                |");
        println!("  That's what this tool tries to find out.                      |");
        println!("                                                                |");
        println!("                                                                |");
        println!("v How does it work? <-------------------------------------------|");
        println!("|                                                                ");
        println!("| This tool works incrementally. It starts by finding the        ");
        println!("| number of permutations that can fit into a string wasting no   ");
        println!("| symbols, then moves on to one symbol, then two, etc.           ");
        println!("|                                                                ");
        println!("| As it does this, it accumulates information that is used to    ");
        println!("| limit the regions of the search space to be explored and       ");
        println!("| guide the search to regions more likely to yield results.      ");
        println!("|                                                                ");
        println!("| The way it actually works is quite complicated and is based    ");
        println!("| on reasoning about upper and lower bounds. For more detail,    ");
        println!("| check out the README in this repository.                       ");
        println!("|                                                                ");
        println!("| Also, check out this blog post, which was the inspiration      ");
        println!("| for this tool: https://tinyurl.com/minimal-superpermutations   ");
        println!("|                                                                ");
        println!("|                                                                ");
        println!("> How do I use it? ---------------------------------------------v");
        println!("                                                                |");
        println!("  You answer a few basic questions, then leave it running...    |");
        println!("                                                                |");
        println!("  ...and running ...and running                                 |");
        println!("                                                                |");
        println!("  For anything above five symbols, it's unlikely the tool will  |");
        println!("  ever finish, but that's ok! We'll find out some things along  |");
        println!("  the way that might be useful.                                 |");
        println!("                                                                |");
        println!("  The tool is very memory and disk hungry, so chances are       |");
        println!("  you'll run out of space before long.                          |");
        println!("                                                                |");
        println!("                                                                |");
        println!("v---------------------------------------------------------------<");
        println!("|                                                                ");
    }

    pub fn print_running() {
        println!("|");
        println!("|");
        println!("> Ok, here we go! --->>>");
    }

    pub fn ask_for_n() -> usize {
        let input = Self::prompt("How many symbols should the string contain?", "5");
        Self::parse_integer(&input)
    }

    pub fn ask_for_memory() -> f64 {
        let input = Self::prompt("How many gigabytes of memory may this tool use?", "12");
        Self::parse_float(&input)
    }

    pub fn ask_for_gzip() -> bool {
        let input = Self::prompt("Do you want to gzip scratch files to save space?", "no");
        Self::parse_boolean(&input)
    }

    pub fn ask_for_verbose() -> bool {
        let input = Self::prompt("Do you want to print verbose output?", "no");
        Self::parse_boolean(&input)
    }

    fn prompt(question: &'static str, default: &'static str) -> String {
        println!("|\n| {} (default: {})", question, default);

        print!(">>> ");
        stdout().flush().ok().expect("Failed to flush stdout.");

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().is_empty() {
            true => default.to_string(),
            false => input,
        }
    }

    fn parse_integer(input: &str) -> usize {
        input.trim().parse().expect("Failed to parse integer.")
    }

    fn parse_float(input: &str) -> f64 {
        input.trim().parse().expect("Failed to parse float.")
    }

    fn parse_boolean(input: &str) -> bool {
        match input.to_lowercase().trim() {
            "y" => true,
            "n" => false,
            "yes" => true,
            "no" => false,
            _ => panic!("Failed to parse boolean."),
        }
    }
}
