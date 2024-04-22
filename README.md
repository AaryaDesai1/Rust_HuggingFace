# Rust_HuggingFace
Rust Serverless Transformer Endpoint

## Objective 
The objective of this project is to create a Rust serverless endpoint that can be used to run HuggingFace transformers. 

## Setup

### Setting up the llm crate
The main issue with this project was trying to set up the llm crate. The llm crate is a Rust wrapper for the HuggingFace transformers library. The llm crate is not yet published on crates.io, so we have to use the git repository.

```toml 
[dependencies]
llm = { git = "https://github.com/rustformers/llm" , branch = "main" }
```

However, when you try to build the project, you will get some dependency errors as shown in the screenshot below. Therefore, we need to make some changes to the `Cargo.toml` file to help solve these issues. 

```bash
tch = { version = "0.3.1", default-features = false }
```
<img width="1117" alt="image" src="https://github.com/AaryaDesai1/Rust_HuggingFace/assets/143753050/c8d47e96-5ebc-4fa2-8fce-7f5c87ef2d2d">

However, even after adding this dependency, you will still get some errors. The error is shown below. 

<img width="920" alt="image" src="https://github.com/AaryaDesai1/Rust_HuggingFace/assets/143753050/c4a967b6-da53-45e9-acbf-7bbf9d644f59">

However, again this presented a whole bunch of issues, which urged me to look into the documentation for the `tch` crate. Based on this, I needed ensure the following steps were completed:
```bash
pip3 install torch torchvision 
pip3 show torch 
```
This will confirm whether you have torch on your machine and will give you the location of the torch installation. Based on the path you find, you can set the `LIBTORCH` environment variable. 
```bash
export LIBTORCH=/path/to/torch
```
You can check whether the environment variable has been set by running the following command:
```bash
echo $LIBTORCH
```                         

After this, I tried to build the project again, and, but still did not work. 
