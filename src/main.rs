use std::collections::VecDeque;

use clap::Parser;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{RngExt, SeedableRng};

// TODO: Config with TOML.
fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Model {
    // _window: Entity,
    random_seed: u64,
    background: Srgba,
    tree: Srgba,
    shadow: Srgba,
    // leaves: Srgba,
}

struct ModelBuilder {
    random_seed: Option<u64>,
    background: Option<Srgba>,
    tree: Option<Srgba>,
    shadow: Option<Srgba>,
}

impl ModelBuilder {
    fn new() -> Self {
        Self {
            random_seed: None,
            background: None,
            tree: None,
            shadow: None,
        }
    }

    fn random_seed(mut self, random_seed: Option<u64>) -> Self {
        if let Some(seed) = random_seed {
            self.random_seed = Some(seed);
        }
        self
    }

    fn background(mut self, background: Option<String>) -> Self {
        if let Some(color) = background {
            if let Ok(background) = Srgba::hex(color) {
                self.background = Some(background);
            }
        }
        self
    }

    fn tree(mut self, tree: Option<String>) -> Self {
        if let Some(color) = tree {
            if let Ok(tree) = Srgba::hex(color) {
                self.tree = Some(tree);
            }
        }
        self
    }

    fn shadow(mut self, shadow: Option<String>) -> Self {
        if let Some(color) = shadow {
            if let Ok(shadow) = Srgba::hex(color) {
                self.shadow = Some(shadow);
            }
        }
        self
    }

    fn build(self) -> Model {
        Model {
            random_seed: self.random_seed.unwrap_or((random_f32() * 100000.0) as u64),
            background: self.background.unwrap_or(Srgba::hex("#75d3e8").unwrap()),
            tree: self.tree.unwrap_or(Srgba::hex("#ffffff").unwrap()),
            shadow: self.tree.unwrap_or(Srgba::hex("#3c84ac").unwrap()),
        }
    }
}

fn model(app: &App) -> Model {
    let args = Cli::parse();
    println!("pattern: {:?}", args.config);
    app.new_window().view(view).build();
    ModelBuilder::new()
        .random_seed(args.seed)
        .background(args.bg)
        .tree(args.tree)
        .shadow(args.shadow)
        .build()
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<std::path::PathBuf>,
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

fn update(_app: &App, _model: &mut Model) {}

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
    let mut rng = StdRng::seed_from_u64(model.random_seed);
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
            .color(model.shadow)
            .points(branch_points(&branch, 2.0));
        draw.polygon()
            .color(model.tree)
            .points(branch_points(&branch, 0.0));
        // Shadow with transparency
        // Stepped shadows
        // Offset shadows
    }
    draw.background().color(model.background);
}
