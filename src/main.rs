use std::error::Error;
use std::fs;
use doommy::Args;
use doommy::ConfigParser;

fn main() -> Result<(), Box<dyn Error>> {
    // Чтение конфигурационного файла
    let file_path = <Args as clap::Parser>::parse().file; // Путь к вашему конфигурационному файлу
    let input = fs::read_to_string(file_path)?;

    // Парсинг входных данных
    let yaml = ConfigParser::parse(&input)?;
    let result = serde_yaml::to_string(&(yaml.0))?;
    println!("{}", result);
    fs::write("result.yaml", result)?;
    Ok(())
}
