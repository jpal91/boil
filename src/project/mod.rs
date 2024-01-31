use std::fs;
use std::process::Command;
use std::io::Write;
use std::path::PathBuf;
use crate::config::ProgType;
use crate::error::BoilResult;


pub fn create_program(path: &PathBuf, p_type: &ProgType) -> BoilResult<()> {
    let mut file = fs::File::create(path)?;

    match p_type {
        ProgType::Python => file.write_all(b"#!/usr/bin/python")?,
        ProgType::Bash => file.write_all(b"#!/bin/bash")?,
        _ => {}
    };

    Ok(())
}

pub fn create_project(path: &PathBuf, p_type: &ProgType) -> BoilResult<()> {
    match p_type {
        ProgType::Python => create_python_proj(path),
        ProgType::Rust => todo!(),
        _ => {
            fs::create_dir_all(path)?;
            Ok(())
        }
    }
    
}

fn create_python_proj(path: &PathBuf) -> BoilResult<()> {
    fs::create_dir_all(path)?;
    let mut dir = path.to_path_buf();
    dir.push(".gitignore");
    
    let mut gitignore = fs::File::create(&dir)?;
    let py_gi = include_bytes!("py_gitignore.in");
    gitignore.write_all(py_gi);

    dir.pop();
    dir.push("src");
    fs::create_dir(&dir)?;

    dir.push("__init__.py");
    fs::File::create(&dir);
    

    Ok(())
}

fn create_rust_project(path: &PathBuf) -> BoilResult<()> {
    Command::new("cargo")
        .args(["new", path.to_str().unwrap()])
        .output()?;
    Ok(())
}