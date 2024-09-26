mod workflow;
mod random;

use clap::Parser;
use random::RandomKind;

pub static AUTHOR_WEBSITE: &str = "hongdenglv.com";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///生成随机数据的类型
    #[arg(short, long)]
    kind: String,
    ///生成随机数据的长度 默认为0 不限制
    #[arg(short, long, default_value_t = 0)]
    length: i8,
    ///生成随机数据的个数
    #[arg(short, long, default_value_t = 9)]
    number: u8,
}

impl Args {
    pub fn new(kind: String, length: i8, number: u8) -> Args {
        Args {
            kind,
            length,
            number,
        }
    }
}

pub struct GenerateRandom {
    kind: RandomKind,
    value: String,
}
impl GenerateRandom {
    pub fn new(kind: RandomKind, value: String) -> GenerateRandom {
        GenerateRandom {
            kind,
            value,
        }
    }
}

fn main() {
    let args = Args::parse();
    let args = Args::new(args.kind, -1, 9);
    let random_kind = RandomKind::get_random_by_name(args.kind);
    // println!("current random kind is {:?} !", random_kind);
    let mut random_value_list = Vec::new();
    for _ in 0..args.number {
        let random_value = random::random_value(random_kind, args.length);
        random_value_list.push(GenerateRandom::new(random_kind, random_value));
    }
    let workflows = workflow::Workflows::new(random_value_list);
    println!("{}", workflows.to_json());
}

