use rig_agent::tools::calculator::{Calculator, CalculatorArgs};
use rig_agent::tools::file_reader::{FileReader, FileReaderArgs};
use rig_agent::tools::web_search::{WebSearch, WebSearchArgs};
use rig_agent::utils::config::Settings;
use rig::tool::Tool;
use tempfile::NamedTempFile;
use std::io::Write;

#[tokio::test]
async fn test_calculator_tool() {
    let calc = Calculator;
    
    let res = calc.call(CalculatorArgs {
        operation: "add".to_string(),
        a: 15.0,
        b: 27.0,
    }).await;
    assert_eq!(res.unwrap(), "42");

    let res_div = calc.call(CalculatorArgs {
        operation: "divide".to_string(),
        a: 100.0,
        b: 0.0,
    }).await;
    assert!(res_div.is_err());
}

#[tokio::test]
async fn test_file_reader_tool() {
    let mut tmp_file = NamedTempFile::new().unwrap();
    write!(tmp_file, "Test content in file").unwrap();
    
    let reader = FileReader;
    let res = reader.call(FileReaderArgs {
        path: tmp_file.path().to_str().unwrap().to_string(),
    }).await.unwrap();
    
    assert_eq!(res, "Test content in file");
}

#[tokio::test]
async fn test_web_search_tool() {
    let search = WebSearch;
    let res = search.call(WebSearchArgs {
        query: "Rust 2024".to_string(),
    }).await.unwrap();
    
    assert!(res.contains("Rust 2024 Edition"));
}

#[test]
fn test_config_loader() {
    let settings = Settings::load();
    assert!(settings.is_ok());
}
