use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use colored::Colorize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    update: bool,
}

fn main() {

    let opt = Opt::from_args();
    if opt.update {
        let _ = update_key_pairs();
        println!("peers list updated");
    }

    let output = Command::new("wg")
    .arg("show")
    .output()
    .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.starts_with("interface") {
            match line.split_once(": ") {
                Some((_key,value)) => {
                println!("{}: {}", "interface".green(), value.green());
                }
                None => {}
            }
        }
        
        else if line.starts_with("peer") {
            match line.split_once(": ") {
                Some((_key,value)) => {
                println!("{}: {}", "peer".yellow(), value.yellow());
                let stored_keys = read_key_value_pairs("/etc/wireguard/peers");
                if let Ok(key_pairs) = stored_keys {
                    for key_pair in key_pairs {
                        if &key_pair.0 == value {
                            println!("  {}: {}", "name".cyan(), &key_pair.1.cyan());
                        }
                    }
                }

                }
                None => {}
            }
        }
        else {
            println!("{}", line);
        }
        
    }

}

fn read_key_value_pairs(file_path: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(':') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    Ok(map)
}


fn update_key_pairs() -> std::io::Result<()> {
    let file = File::open("/etc/wireguard/wg0.conf")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut output_file = File::create("/etc/wireguard/peers")?;
    let mut buffer = String::new();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.starts_with("#name = ") {
            let name_only = line.split("#name = ").collect::<String>();
            if let Some(next_line) = lines.next() {
                let key_only = next_line?.split("PublicKey = ").collect::<String>();
                buffer.push_str(&(key_only + ":" + &name_only));
            }
        writeln!(output_file, "{}", buffer.trim())?;
        }
        buffer.clear();
    }

    Ok(())
}




