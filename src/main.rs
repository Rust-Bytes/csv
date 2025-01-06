use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

fn extract_domain(email: &str) -> Option<String> {
    let parts = email.split("@").collect::<Vec<&str>>();

    if parts.len() == 2 {
        Some(parts[1].to_string())
    } else {
        None
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let csv_name = "users.csv".to_string();
    let data = File::open(csv_name)?;

    let mut rdr = csv::Reader::from_reader(data);

    let mut all_domains: HashSet<String> = HashSet::new();

    for result in rdr.records() {
        let record = result?;

        let email = record[1].to_string();
        match extract_domain(&email) {
            Some(val) => {
                all_domains.insert(val.clone());
            }
            None => {
                println!("No domain extracted!")
            }
        }
    }

    for domain in all_domains.iter().enumerate() {
        println!("Domain: {}, the {}nth", domain.1, domain.0);
    }

    println!("Total Unique domain emails: {}", all_domains.len());

    Ok(())
}
