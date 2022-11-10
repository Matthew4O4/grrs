use std::io::{self, Write};
use std::ptr::null;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;
use grre::find_matches;

fn main()  -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("cloud not read file '{}'", args.path.display()))?;
    find_matches(&content, &args.pattern,&std::io::stdout());
    Ok(())
}

#[test]
fn find_a_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\nodlor sit amet", "lorem", &mut result);
    // b前缀使其成为字节字符串文本，以它的类型是&[u8]，而不是&str。
    assert_eq!(result,b"lorem ipsum\n");
}


// search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // 模式、要查找的字符串
    pattern: String,
    // 路径:： PathBuf就像一个String，但用作跨平台工作的文件系统路径。
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // 旁注： （StructOpt）有很多自定义属性可以添加到字段中。
    // 例如，我们添加了一个PathBuf类型，让 structopt 解析。
    // 想在后面的参数中，使用此字段 -o或 --output，您可以添加#[structopt(short = "o", long = "output")]
}


