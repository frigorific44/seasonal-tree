use std::fs;

use crate::color::Color;
use clap::Parser;
use hex_color::HexColor;
use serde::Deserialize;

#[derive(Debug)]
pub struct Model {
    pub output: Option<std::path::PathBuf>,
    pub random_seed: Option<u64>,
    pub background: Color,
    pub tree: Color,
    pub shadow: Color,
    pub smoothness: f32,
    pub shadow_offset: f32,
}

#[derive(Debug, Deserialize)]
struct ModelBuilder {
    output: Option<std::path::PathBuf>,
    random_seed: Option<u64>,
    background: Option<Color>,
    tree: Option<Color>,
    shadow: Option<Color>,
    smoothness: Option<f32>,
    shadow_offset: Option<f32>,
}

impl ModelBuilder {
    fn new() -> Self {
        Self {
            output: None,
            random_seed: None,
            background: None,
            tree: None,
            shadow: None,
            smoothness: None,
            shadow_offset: None,
        }
    }

    fn output(mut self, output: Option<std::path::PathBuf>) -> Self {
        if output.is_some() {
            self.output = output;
        }
        self
    }

    fn random_seed(mut self, random_seed: Option<u64>) -> Self {
        if let Some(seed) = random_seed {
            self.random_seed = Some(seed);
        }
        self
    }

    fn background(mut self, background: Option<String>) -> Self {
        if let Some(color_str) = background {
            if let Ok(background) = HexColor::parse_rgb(&color_str) {
                self.background = Some(background.into());
            }
        }
        self
    }

    fn tree(mut self, tree: Option<String>) -> Self {
        if let Some(color_str) = tree {
            if let Ok(tree) = HexColor::parse_rgb(&color_str) {
                self.tree = Some(tree.into());
            }
        }
        self
    }

    fn shadow(mut self, shadow: Option<String>) -> Self {
        if let Some(color_str) = shadow {
            if let Ok(shadow) = HexColor::parse_rgb(&color_str) {
                self.shadow = Some(shadow.into());
            }
        }
        self
    }

    fn smoothness(mut self, smoothness: Option<f32>) -> Self {
        if let Some(smoothness) = smoothness {
            self.smoothness = Some(smoothness);
        }
        self
    }

    fn shadow_offset(mut self, shadow_offset: Option<f32>) -> Self {
        if let Some(offset) = shadow_offset {
            self.shadow_offset = Some(offset);
        }
        self
    }

    fn build(self) -> Model {
        Model {
            output: self.output,
            random_seed: self.random_seed,
            background: self.background.unwrap_or(Color::rgb(117, 211, 232)),
            tree: self.tree.unwrap_or(Color::rgb(255, 255, 255)),
            shadow: self.shadow.unwrap_or(Color::rgb(60, 132, 172)),
            smoothness: self.smoothness.unwrap_or(0.5),
            shadow_offset: self.shadow_offset.unwrap_or(2.0),
        }
    }
}

pub fn model() -> Model {
    let args = Cli::parse();
    let mut builder = ModelBuilder::new();
    if let Some(config) = args.config {
        match fs::read_to_string(config) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => builder = config,
                Err(e) => println!("error parsing config: {e:?}"),
            },
            Err(e) => println!("error reading file: {e:?}"),
        }
    }
    builder
        .output(args.out)
        .random_seed(args.seed)
        .background(args.bg)
        .tree(args.tree)
        .shadow(args.shadow)
        .smoothness(args.smoothness)
        .shadow_offset(args.shadow_offset)
        .build()
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "CONFIG_FILE")]
    config: Option<std::path::PathBuf>,
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    out: Option<std::path::PathBuf>,
    #[arg(short, long)]
    seed: Option<u64>,
    #[arg(long)]
    bg: Option<String>,
    #[arg(long)]
    tree: Option<String>,
    #[arg(long)]
    shadow: Option<String>,
    #[arg(long)]
    smoothness: Option<f32>,
    #[arg(long)]
    shadow_offset: Option<f32>,
}
