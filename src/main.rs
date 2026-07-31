use crate::point::Point;
use std::collections::VecDeque;

use clap::Parser;
use hex_color::HexColor;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use tiny_skia::Pixmap;

pub mod point;

const PI: f32 = 3.1415926535897932384626433;

// TODO: Config with TOML.
fn main() {
    view(&model());
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

impl From<Color> for tiny_skia::Color {
    fn from(value: Color) -> Self {
        tiny_skia::Color::from_rgba8(value.r, value.g, value.b, 255)
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

fn model() -> Model {
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

struct BoundaryNode {
    point: Point,
    size: f32,
    length: f32,
    angle: f32,
}

impl BoundaryNode {
    fn grow(&self, rng: &mut StdRng) -> Option<BoundaryNode> {
        let new_point = (Point::from_angle(self.angle) * self.length) + self.point;
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

fn branch_points(branch: &Vec<BoundaryNode>, offset: f32) -> VecDeque<Point> {
    let mut points: VecDeque<Point> = VecDeque::new();

    if let Some(start) = branch.first() {
        let shell = Point::from_angle(start.angle + (PI / 2.0)).normalize();
        points.push_front(start.point + (shell * (start.size + offset)));
        points.push_back(start.point + (shell * (start.size + offset) * -1.0));
    }
    for (i, w) in branch.windows(2).enumerate() {
        let shell = Point::from_angle((w[0].angle + w[1].angle + PI) / 2.0).normalize();
        let mut cap = Point::ZERO;
        if i == branch.len() - 2 {
            cap = Point::from_angle((w[0].angle + w[1].angle) / 2.0).normalize() * offset;
        }
        points.push_front(w[1].point + cap + (shell * (w[1].size + offset)));
        points.push_back(w[1].point + cap + (shell * (w[1].size + offset) * -1.0));
    }
    points
}

fn view(model: &Model) {
    let mut rng: StdRng;
    if let Some(seed) = model.random_seed {
        rng = StdRng::seed_from_u64(seed)
    } else {
        rng = rand::make_rng()
    }
    let width: u32 = 1920;
    let height: u32 = 1080;
    let mut pixmap = Pixmap::new(width, height).unwrap();

    pixmap.fill(model.background.into());

    let root = BoundaryNode {
        point: Point::new((width / 2) as f32, (1080) as f32),
        size: 32.0,
        length: 60.0,
        angle: -PI / 2.0,
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

        let mut shadow_pb = tiny_skia::PathBuilder::new();
        let mut shadow_branch = branch_points(&branch, 2.0);
        if let Some(first) = shadow_branch.pop_front() {
            shadow_pb.move_to(first.x, first.y);
        }
        for p in shadow_branch {
            shadow_pb.line_to(p.x, p.y);
        }
        shadow_pb.close();
        let mut shadow_paint = tiny_skia::Paint::default();
        shadow_paint.set_color(model.shadow.into());
        shadow_paint.anti_alias = true;
        pixmap.fill_path(
            &shadow_pb.finish().unwrap(),
            &shadow_paint,
            tiny_skia::FillRule::Winding,
            tiny_skia::Transform::from_translate(0.0, 0.0),
            None,
        );

        let mut tree_pb = tiny_skia::PathBuilder::new();
        let mut tree_branch = branch_points(&branch, 0.0);
        if let Some(first) = tree_branch.pop_front() {
            tree_pb.move_to(first.x, first.y);
        }
        for p in tree_branch {
            tree_pb.line_to(p.x, p.y);
        }
        tree_pb.close();
        let mut tree_paint = tiny_skia::Paint::default();
        tree_paint.set_color(model.tree.into());
        tree_paint.anti_alias = true;
        pixmap.fill_path(
            &tree_pb.finish().unwrap(),
            &tree_paint,
            tiny_skia::FillRule::Winding,
            tiny_skia::Transform::from_translate(0.0, 0.0),
            None,
        );
    }
    if let Some(output_path) = &model.output {
        pixmap.save_png(output_path.clone()).unwrap();
    }
}
