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
    // leaves: Srgba,
    // shadow: Srgba,
}

struct ModelBuilder {
    random_seed: Option<u64>,
    background: Option<Srgba>,
    tree: Option<Srgba>,
}

impl ModelBuilder {
    fn new() -> Self {
        Self {
            random_seed: None,
            background: None,
            tree: None,
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

    fn build(self) -> Model {
        Model {
            random_seed: self.random_seed.unwrap_or((random_f32() * 100000.0) as u64),
            background: self.background.unwrap_or(Srgba::hex("#75d3e8").unwrap()),
            tree: self.tree.unwrap_or(Srgba::hex("#ffffff").unwrap()),
        }
    }
}

fn model(app: &App) -> Model {
    let args = Cli::parse();
    println!("pattern: {:?}", args.config);
    let _window = app.new_window().view(view).build();
    ModelBuilder::new()
        .random_seed(args.seed)
        .background(args.background)
        .tree(args.tree)
        .build()
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<std::path::PathBuf>,
    #[arg(short, long)]
    seed: Option<u64>,
    #[arg(short, long)]
    background: Option<String>,
    #[arg(short, long)]
    tree: Option<String>,
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
        let new_size = self.size - 1.0;
        if new_size < 10.0 {
            return None;
        }
        let new_angle = self.angle;
        Some(BoundaryNode {
            point: new_point,
            size: new_size,
            length: self.length,
            angle: new_angle + rng.random_range(-0.4..0.4),
        })
    }
}

fn update(_app: &App, _model: &mut Model) {}

fn view(app: &App, _model: &Model) {
    let win = app.window_rect();
    let mut rng = StdRng::seed_from_u64(_model.random_seed);
    let draw = app.draw();

    let root = BoundaryNode {
        point: pt2(rng.random_range(win.left()..win.right()), win.bottom()),
        size: 32.0,
        length: 40.0,
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
            branch.push(curr);
            curr = next;
        }

        branches.push(branch);
    }
    for branch in branches {
        if branch.len() < 2 {
            continue;
        }
        let mut points: VecDeque<Vec2> = VecDeque::new();

        for w in branch.windows(2) {
            let offset = Vec2::from_angle((w[0].angle + w[1].angle + PI) / 2.0).normalize();
            points.push_front(w[1].point + (offset * w[1].size));
            points.push_back(w[1].point + (offset * w[1].size * -1.0));
        }
        // let points = branch.iter().map(|node| node.point);
        // draw.polyline().weight(1.0).points(points)
        draw.polygon().color(_model.tree).points(points);
        // draw.polyline().weight(5.0).points(points).color(RED);
        // Shadow
        // Shadow with transparency
        // Stepped shadows
        // Offset shadows
    }
    draw.background().color(_model.background);
}
