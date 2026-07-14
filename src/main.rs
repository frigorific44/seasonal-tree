use std::collections::VecDeque;

use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{RngExt, SeedableRng};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: Entity,
    random_seed: u64,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build();
    Model {
        _window,
        random_seed: (random_f32() * 100000.0) as u64,
    }
}

struct BoundaryNode {
    point: Vec2,
    size: f32,
    length: f32,
    angle: f32,
}

// let next_point = (Vec2::from_angle(base.angle) * length) + base.point;
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
            angle: new_angle + rng.random_range(-0.2..0.2),
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
        length: 20.0,
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
            // let next = curr.grow();
            let next = match curr.grow(&mut rng) {
                Some(x) => x,
                None => break,
            };
            branch.push(curr);
            curr = next;
            // branch.push(tree_node(next_point, size))
        }

        branches.push(branch);
    }
    for branch in branches {
        let points = branch.iter().map(|node| node.point);
        // draw.polyline().weight(1.0).points(points)
        draw.polyline().weight(1.0).points(points).color(RED);
    }
    draw.background().color(PLUM);
}
