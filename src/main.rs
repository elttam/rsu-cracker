pub mod util;
pub mod java_random;
pub mod java_random_crack;
pub mod random_string_utils;
pub mod random_string_utils_crack;

use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Parser, Subcommand};
use random_string_utils::RandomStringUtils;
use random_string_utils_crack::recover_seed;
use java_random::JavaRandom;
use java_random_crack::recover_seed as java_recover_seed;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    RandomAlphanumeric {
        /// Output of RandomStringUtils.randomAlphanumeric(n)
        token: String,

        /// Number of tokens to output
        #[arg(short, long, default_value_t = 10)]
        count: u16,

        /// Length of tokens to output
        #[arg(short, long, default_value_t = 0)]
        output_len: u16,
    },
    NextInt {
        /// Outputs of random.nextInt(n)
        #[arg(use_value_delimiter = true, value_delimiter = ' ')]
        outputs: Vec<u128>,

        /// Value of the bound n
        #[arg(short, long)]
        n: u128,

        /// Number of values to output
        #[arg(short, long, default_value_t = 10)]
        count: u16,
    }
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::RandomAlphanumeric { token, count, output_len }) => {
            let token_len = token.len();
            if token_len < 9 {
                eprintln!("[!] Token length is {}, but a token of at least 9 characters is needed. Results may be incorrect.", token_len);
            }

            let outputs = token.bytes().map(|b| (b - 32) as u128).collect::<Vec<u128>>();
            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let seed = recover_seed(outputs);
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            eprintln!("\n");
            eprintln!("[*] Finished running after {}s", ((end - start) as f64)/1000.0);

            if seed.is_none() {
                eprintln!("[-] Could not recover seed!");
            } else {
                eprintln!("[+] Java Random seed recovered: {}", seed.unwrap());
                let mut rsu = RandomStringUtils::new(seed.unwrap());
                rsu.random_alphanumeric(token_len);
                if *count == 1 {
                    eprintln!("[+] The next token is:");
                } else {
                    eprintln!("[+] The next {} tokens are:", count);
                }
                let l = if *output_len == 0 { token_len } else { *output_len as usize };
                for _ in 0..*count {
                    println!("{}", rsu.random_alphanumeric(l));
                }
            }
        },
        Some(Commands::NextInt { outputs, n, count }) => {
            let need = (48. / (*n as f32).log2()).ceil() as usize;
            if outputs.len() < need {
                eprintln!("[!] Number of outputs is {}, but at least {} are needed. Results may be incorrect", outputs.len(), need);
            }

            if *n <= 6 {
                eprintln!("[!] Bound is very small, recovery may not work.");
            }

            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let seed = java_recover_seed(outputs.clone(), *n);
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            eprintln!("\n");
            eprintln!("[*] Finished running after {}s", ((end - start) as f64)/1000.0);

            if seed.is_none() {
                eprintln!("[-] Could not recover seed!");
            } else {
                eprintln!("[+] Java Random seed recovered: {}", seed.unwrap());
                let mut rand = JavaRandom::new(seed.unwrap());
                for _ in 0..outputs.len() {
                    rand.next_int(*n);
                }
                if *count == 1 {
                    eprintln!("[+] The next output is:");
                } else {
                    eprintln!("[+] The next {} outputs are:", count);
                }
                for _ in 0..*count {
                    println!("{}", rand.next_int(*n));
                }
            }
        },
        None => {}
    }
}
