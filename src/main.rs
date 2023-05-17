#[allow(unused_imports)]
use anyhow::{Context, Result};
use git_starter_rust::cli::{
    Cli, CloneRepOptions, Commands, CommitTreeOptions, CreateBlobOptions, ReadBlobOptions,
    ReadTreeOptions,
};
use git_starter_rust::*;
//use std::env;
use clap::Parser;
use git_starter_rust::clone::clone_repo;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            if let Err(err) = git_init() {
                eprintln!("ERROR in Init operation: {}", err);
                std::process::exit(1);
            }
        }
        Commands::CatFile(read_options) => {
            if let Err(err) = read_git_object(ReadBlobOptions::read(read_options)?) {
                eprintln!("ERROR in Read a blob object: {}", err);
                std::process::exit(2);
            }
        }
        Commands::HashObject(file) => {
            let file_data = fs::read(CreateBlobOptions::read(file)?);
            if let Err(err) = file_data {
                eprintln!("ERROR in Create a blob object: {}", err);
                std::process::exit(3);
            }
            let sha1_out =
                write_git_object_target_dir("blob", &file_data.unwrap(), ".git/objects/");
            if let Err(err) = sha1_out {
                eprintln!("ERROR in Create a blob object: {}", err);
                std::process::exit(4);
            }
            println!("{}", sha1_out.unwrap());
        }
        Commands::LsTree(hash) => {
            let result = read_tree(ReadTreeOptions::read(hash)?);
            if let Err(err) = result {
                eprintln!("ERROR in Read a tree object: {}", err);
                std::process::exit(5);
            }
            for s in result.unwrap() {
                stdout().write_all(s.as_slice())?;
                stdout().write_all(&[b'\n'])?;
            }
        }
        Commands::WriteTree => {
            let sha1_out = write_tree(&PathBuf::from("."));
            if let Err(err) = sha1_out {
                eprintln!("ERROR in Write a tree object: {}", err);
                std::process::exit(6);
            }
            print!("{}", sha1_out.unwrap());
        }
        Commands::CommitTree(args) => {
            let tree = create_commit(CommitTreeOptions::read(args)?);
            if let Err(err) = tree {
                eprintln!("ERROR in Commit a git object: {}", err);
                std::process::exit(7);
            }
            println!("{}", tree.unwrap());
        }
        Commands::Clone(args) => {
            if let Err(err) = clone_repo(CloneRepOptions::read(args)?) {
                eprintln!("ERROR in Clone git object: {}", err);
                std::process::exit(8);
            }
        }
    }
    Ok(())
}

/****************************************************************************************************************
 * **************************************************************************************************************
*/
