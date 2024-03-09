use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// 输入csv文件路径
    #[arg(value_name = "输入文件")]
    input: String,
    /// 输出txt文件路径
    #[arg(value_name = "输出文件")]
    output: Option<String>,
    /// csv文件是否包含表头
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    header: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    pub search: String,
    pub replace: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(cli.header)
        .from_path(&cli.input)?;

    let mut rules: Vec<Rule> = Vec::new();
    for result in rdr.deserialize() {
        let rule: Rule = result?;
        rules.push(rule);
    }

    let output_file = cli.output.unwrap_or_else(|| {
        let path = PathBuf::from(&cli.input);
        let fname = path.file_stem().unwrap().to_str().unwrap().to_string();
        format!("{}.txt", fname)
    });
    let mut writer = File::create(&output_file)?;
    // BOM编码
    writer.write_all(&[0xEF, 0xBB, 0xBF])?;
    for rule in rules {
        writer.write_all(
            format!(
                r#"StartRule
Search={}
Replace={}
Pattern=%REPLACE% %ORIG%
select=0
mode=0
EndRule

"#,
                rule.search, rule.replace
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}
