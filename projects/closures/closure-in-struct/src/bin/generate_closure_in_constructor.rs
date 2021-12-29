struct Node {
    x: u32,
    y: u32,
}

struct Distance
{
    pub distance: Box<dyn Fn(Node, Node) -> f32>,
}

impl Distance
{
    fn new() -> Distance {
        Distance {
            distance: Box::new(
                |a: Node, b: Node| -> f32 {
                    f32::sqrt(((b.x - a.x).pow(2) + (b.y - a.y).pow(2)) as f32)
                }
            )
        }
    }
}

fn main() {
    let n1 = Node {x: 0, y: 0};
    let n2 = Node {x: 3, y: 4};
    let obj = Distance::new();
    println!("{:?}", (obj.distance)(n1, n2));
}
