use std::collections::VecDeque;

use clap::Parser;
use hex_color::HexColor;
use nannou::prelude::{App, Srgba, Vec2, pt3};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

const PI: f32 = 3.1415926535897932384626433;

// TODO: Config with TOML.
fn main() {
    nannou::app(model).run();
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl From<HexColor> for Color {
    fn from(value: HexColor) -> Self {
        Color::rgb(value.r, value.g, value.b)
    }
}

impl From<Color> for Srgba {
    fn from(value: Color) -> Self {
        Srgba::rgb_u8(value.r, value.g, value.b)
    }
}

#[derive(Debug)]
struct Model {
    // _window: Entity,
    output: Option<std::path::PathBuf>,
    random_seed: Option<u64>,
    background: Color,
    tree: Color,
    shadow: Color,
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

fn model(app: &App) -> Model {
    let args = Cli::parse();
    println!("pattern: {:?}", args.config);
    app.new_window().view(view).build();
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

struct BoundaryNode {
    point: Vec2,
    size: f32,
    length: f32,
    angle: f32,
}

impl BoundaryNode {
    fn grow(&self, rng: &mut StdRng) -> Option<BoundaryNode> {
        let new_point = (Vec2::from_angle(self.angle) * self.length) + self.point;
        let new_size = self.size - 1.7;
        if new_size < 1.0 {
            return None;
        }
        let new_angle = self.angle + rng.random_range(-0.4..0.4);
        Some(BoundaryNode {
            point: new_point,
            size: new_size,
            length: self.length,
            angle: new_angle,
        })
    }

    fn diverge(&self, rng: &mut StdRng) -> BoundaryNode {
        let new_angle = self.angle + rng.random_range(-0.6..0.6);
        BoundaryNode {
            point: self.point,
            size: self.size,
            length: self.length,
            angle: new_angle,
        }
    }
}

fn branch_points(branch: &Vec<BoundaryNode>, offset: f32) -> VecDeque<Vec2> {
    let mut points: VecDeque<Vec2> = VecDeque::new();

    if let Some(start) = branch.first() {
        let shell = Vec2::from_angle(start.angle + (PI / 2.0)).normalize();
        points.push_front(start.point + (shell * (start.size + offset)));
        points.push_back(start.point + (shell * (start.size + offset) * -1.0));
    }
    for (i, w) in branch.windows(2).enumerate() {
        let shell = Vec2::from_angle((w[0].angle + w[1].angle + PI) / 2.0).normalize();
        let mut cap = Vec2::ZERO;
        if i == branch.len() - 2 {
            cap = Vec2::from_angle((w[0].angle + w[1].angle) / 2.0).normalize() * offset;
        }
        points.push_front(w[1].point + cap + (shell * (w[1].size + offset)));
        points.push_back(w[1].point + cap + (shell * (w[1].size + offset) * -1.0));
    }
    points
}

fn view(app: &App, model: &Model) {
    let win = app.window_rect();
    let mut rng: StdRng;
    if let Some(seed) = model.random_seed {
        rng = StdRng::seed_from_u64(seed)
    } else {
        rng = rand::make_rng()
    }
    let draw = app.draw();

    let root = BoundaryNode {
        point: win.mid_bottom(),
        size: 32.0,
        length: 60.0,
        angle: PI / 2.0,
    };
    let mut boundary: VecDeque<BoundaryNode> = VecDeque::new();
    boundary.push_back(root);
    let mut branches: Vec<Vec<BoundaryNode>> = Vec::new();
    // If this handles nodes added during, I'll be grateful.
    loop {
        //TODO: Would an iterator work?
        let base = boundary.pop_front();
        let mut curr = match base {
            Some(x) => x,
            None => break,
        };
        // Branch preparation.
        let mut branch: Vec<BoundaryNode> = Vec::new();
        // Branch generation.
        loop {
            let next = match curr.grow(&mut rng) {
                Some(x) => x,
                None => break,
            };
            if rng.random_ratio(1, 5) {
                boundary.push_back(curr.diverge(&mut rng));
            }
            branch.push(curr);
            curr = next;
        }

        branches.push(branch);
    }
    for branch in branches.iter().rev() {
        if branch.len() < 2 {
            continue;
        }
        draw.translate(pt3(0.0, 0.0, 0.0))
            .polygon()
            .color(Srgba::from(model.shadow))
            .points(branch_points(&branch, 2.0));
        draw.polygon()
            .color(Srgba::from(model.tree))
            .points(branch_points(&branch, 0.0));
        // Shadow with transparency
        // Stepped shadows
        // Offset shadows
    }
    draw.background().color(Srgba::from(model.background));
    if let Some(output_path) = &model.output {
        app.main_window().save_screenshot(output_path.clone());
    }
}
