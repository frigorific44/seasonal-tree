use crate::color::Color;
use crate::model::{Model, model};
use crate::point::Point;
use std::collections::VecDeque;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use tiny_skia::Pixmap;

pub mod color;
pub mod model;
pub mod point;

const PI: f32 = 3.1415926535897932384626433;

// TODO: Config with TOML.
fn main() {
    view(&model());
}

trait Drawable {
    fn compose(&self, surface: &mut Pixmap);
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

struct Branch {
    nodes: Vec<BoundaryNode>,
    translation: Point,
    offset: f32,
    color: Color,
}

impl Branch {
    fn new(nodes: Vec<BoundaryNode>) -> Self {
        Branch {
            nodes,
            translation: Point::new(0.0, 0.0),
            offset: 0.0,
            color: Color::rgb(0, 0, 0),
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }
}
impl Drawable for Branch {
    fn compose(&self, surface: &mut Pixmap) {
        let mut branch_pb = tiny_skia::PathBuilder::new();

        let mut points: VecDeque<Point> = VecDeque::new();
        if let Some(start) = self.nodes.first() {
            let shell = Point::from_angle(start.angle + (PI / 2.0)).normalize();
            let left = start.point + (shell * (start.size + self.offset));
            let right = start.point + (shell * (start.size + self.offset) * -1.0);
            let l_a = left;
            let l_b = left;
            let r_a = right;
            let r_b = right;
            points.push_front(l_b);
            points.push_front(l_a);
            points.push_back(r_b);
            points.push_back(r_a);
        }
        for (i, w) in self.nodes.windows(2).enumerate() {
            let shell = Point::from_angle((w[0].angle + w[1].angle + PI) / 2.0).normalize();
            let mut cap = Point::ZERO;
            if i == self.nodes.len() - 2 {
                cap = Point::from_angle((w[0].angle + w[1].angle) / 2.0).normalize() * self.offset;
            }
            let left = w[1].point + cap + (shell * (w[1].size + self.offset));
            let right = w[1].point + cap + (shell * (w[1].size + self.offset) * -1.0);
            let l_a = left;
            let l_b = left;
            let l_c = left;
            let r_a = right;
            let r_b = right;
            let r_c = right;
            points.push_front(l_b);
            points.push_front(l_a);
            points.push_front(l_c);
            points.push_back(r_b);
            points.push_back(r_a);
            points.push_back(r_c);
        }

        if let Some(first) = points.pop_front() {
            branch_pb.move_to(first.x, first.y);
        }
        points.make_contiguous();
        for c in points.as_slices().0.chunks(3) {
            branch_pb.cubic_to(c[0].x, c[0].y, c[1].x, c[1].y, c[2].x, c[2].y);
        }
        branch_pb.close();
        surface.fill_path(
            &branch_pb.finish().unwrap(),
            &self.color.into(),
            tiny_skia::FillRule::Winding,
            tiny_skia::Transform::from_translate(self.translation.x, self.translation.y),
            None,
        );
    }
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
        point: Point::new((width / 2) as f32, (height) as f32),
        size: 32.0,
        length: 60.0,
        angle: -PI / 2.0,
    };
    let mut boundary: VecDeque<BoundaryNode> = VecDeque::new();
    boundary.push_back(root);
    let mut branches: Vec<Branch> = Vec::new();
    loop {
        //TODO: Would an iterator work?
        let base = boundary.pop_front();
        let mut curr = match base {
            Some(x) => x,
            None => break,
        };
        // Branch preparation.
        let mut branch_nodes: Vec<BoundaryNode> = Vec::new();
        // Branch generation.
        loop {
            let next = match curr.grow(&mut rng) {
                Some(x) => x,
                None => break,
            };
            if rng.random_ratio(1, 5) {
                boundary.push_back(curr.diverge(&mut rng));
            }
            branch_nodes.push(curr);
            curr = next;
        }

        branches.push(Branch::new(branch_nodes));
    }
    for branch in branches.iter_mut().rev() {
        if branch.len() < 2 {
            continue;
        }
        branch.offset = 2.0;
        branch.color = model.shadow;
        branch.compose(&mut pixmap);

        branch.offset = 0.0;
        branch.color = model.tree;
        branch.compose(&mut pixmap);
    }
    if let Some(output_path) = &model.output {
        pixmap.save_png(output_path.clone()).unwrap();
    }
}
