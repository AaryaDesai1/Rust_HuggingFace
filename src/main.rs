// Writing a main.rs that uses a hugging face llm model
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use tch::nn::ModuleT;
use tch::{nn, Device, Tensor};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} model-path", args[0]);
        std::process::exit(1);
    }
    let model_path = &args[1];
    let device = Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let model = tch::BertConfig::new(&vs.root()).load(model_path)?;

    let mut input = Tensor::of_slice(&[0i64]);
    input = input.to_device(device);
    let output = model.forward_t(Some(&input), None, None, None, None, None, None, false)?;
    println!("{:?}", output);
    Ok(())
}


