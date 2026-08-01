use crate::color::Color;
use clap::Parser;
use hex_color::HexColor;

#[derive(Debug)]
pub struct Model {
    pub output: Option<std::path::PathBuf>,
    pub random_seed: Option<u64>,
    pub background: Color,
    pub tree: Color,
    pub shadow: Color,
    // leaves: Srgba,
}

struct ModelBuilder {
    output: Option<std::path::PathBuf>,
    random_seed: Option<u64>,
    background: Option<Color>,
    tree: Option<Color>,
    shadow: Option<Color>,
}

impl ModelBuilder {
    fn new() -> Self {
        Self {
            output: None,
            random_seed: None,
            background: None,
            tree: None,
            shadow: None,
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

    fn build(self) -> Model {
        Model {
            output: self.output,
            random_seed: self.random_seed,
            background: self.background.unwrap_or(Color::rgb(117, 211, 232)),
            tree: self.tree.unwrap_or(Color::rgb(255, 255, 255)),
            shadow: self.shadow.unwrap_or(Color::rgb(60, 132, 172)),
        }
    }
}

pub fn model() -> Model {
    let args = Cli::parse();
    println!("pattern: {:?}", args.config);
    ModelBuilder::new()
        .output(args.out)
        .random_seed(args.seed)
        .background(args.bg)
        .tree(args.tree)
        .shadow(args.shadow)
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
}
