use base64::prelude::*;
use clap::{Arg, ArgGroup, Command, Parser, ValueEnum};
use hex;
use std::{fs, io::Write, string};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PipelineProgram {
    B64enc,
    B64dec,
    Hex2bin,
    Bin2hex,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct InputTarget {
    #[arg(short = 'f')]
    in_file: Option<String>,

    #[arg(short = 'i')]
    in_string: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    pipeline: Vec<PipelineProgram>,

    #[command(flatten)]
    input: InputTarget,

    #[arg(long = "output")]
    out_file: Option<String>,
}

fn get_input(args: &Args) -> String {
    let text = args
        .input
        .in_file
        .clone()
        .or(args.input.in_string.clone())
        .unwrap();
    let is_file = args.input.in_file.is_some();

    if is_file {
        // read file
        fs::read_to_string(&text).expect(&format!("Could not find file {}", text))
    } else {
        text
    }
}

trait Handler {
    fn handle(buffer: &Vec<u8>) -> Vec<u8>;
}

struct B64dec {}
impl Handler for B64dec {
    fn handle(buffer: &Vec<u8>) -> Vec<u8> {
        BASE64_STANDARD.decode(buffer).unwrap()
    }
}

struct B64enc {}
impl Handler for B64enc {
    fn handle(buffer: &Vec<u8>) -> Vec<u8> {
        BASE64_STANDARD.encode(buffer).into_bytes()
    }
}

struct Hex2bin {}
impl Handler for Hex2bin {
    fn handle(buffer: &Vec<u8>) -> Vec<u8> {
        let s = String::from_utf8(buffer.clone()).unwrap();
        hex::decode(s).unwrap()
    }
}

struct Bin2hex {}
impl Handler for Bin2hex {
    fn handle(buffer: &Vec<u8>) -> Vec<u8> {
        hex::encode(buffer).into_bytes()
    }
}

fn do_pipeline(input: Vec<u8>, args: &Args) -> Vec<u8> {
    let mut val = input;
    let pipeline = args.pipeline.clone();
    for el in pipeline {
        val = match el {
            PipelineProgram::B64enc => B64enc::handle(&val),
            PipelineProgram::B64dec => B64dec::handle(&val),
            PipelineProgram::Hex2bin => Hex2bin::handle(&val),
            PipelineProgram::Bin2hex => Bin2hex::handle(&val),
        }
    }

    val
}

fn main() {
    let args = Args::parse();
    let input_text = get_input(&args);

    let out = do_pipeline(input_text.into_bytes(), &args);

    if args.out_file.is_some() {
        let mut f = fs::File::create(args.out_file.unwrap()).unwrap();
        f.write_all(&out).unwrap();
    } else {
        let stringified = String::from_utf8(out.clone());

        if stringified.is_ok() {
            println!("{:?}", stringified.unwrap());
        } else {
            println!("{:?}", out)
        }
    }
}
