fn main() {
    let country_code = 49;
    let country = match country_code {
        49 => "Germany",
        48 => "Poland",
        46 => "Sweden",
        1..=999 => "Unknown", // range inclusive
        _ => "Invalid",
    };
    println!("country code with {} is {}", country_code, country)
}
