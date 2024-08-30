use std::rc::Rc;

type Signal = f64;

struct Error;

type Result<T> = std::result::Result<T, Error>;

struct Interface {
    pub inputs: usize,
    pub outputs: usize,
}

trait Module {
    fn interface(&self) -> Interface;

    fn process(&mut self, inputs: &[Signal], outputs: &mut [Signal]) -> Result<()>;
}

struct Const {
    value: Signal,
}

impl Module for Const {
    fn interface(&self) -> Interface {
        Interface {
            inputs: 0,
            outputs: 1,
        }
    }

    fn process(&mut self, _: &[Signal], outputs: &mut [Signal]) -> Result<()> {
        outputs[0] = self.value;
        Ok(())
    }
}

struct PrintLn;

impl Module for PrintLn {
    fn interface(&self) -> Interface {
        Interface {
            inputs: 1,
            outputs: 0,
        }
    }

    fn process(&mut self, inputs: &[Signal], _: &mut [Signal]) -> Result<()> {
        println!("{}", inputs[0]);
        Ok(())
    }
}

struct Wire {
    from: (Rc<Node>, usize),
    to: (Rc<Node>, usize),
}

struct Node {
    module: Box<dyn Module>,
    input: Vec<Signal>,
    output: Vec<Signal>,
}

struct Rack {
    nodes: Vec<Rc<Node>>,
    wires: Vec<Wire>,
}

impl Rack {
    fn new() -> Self {
        Rack {
            nodes: Vec::new(),
            wires: Vec::new(),
        }
    }

    fn add_module<M: Module + 'static>(&mut self, module: M) -> Rc<Node> {
        let interface = module.interface();
        let node = Rc::new(
            Node {
                module: Box::new(module),
                input: vec![0.0; interface.inputs],
                output: vec![0.0; interface.outputs],
            }
        );
        self.nodes.push(node.clone());
        node
    }

    fn add_wire(&mut self, wire: Wire) {
        self.wires.push(wire)
    }

    fn process(&mut self, node: Rc<Node>) {
        todo!()
    }
}

fn main() {
    let mut rack = Rack::new();

    let n1 = rack.add_module(Const { value: 2.0 });
    let n2 = rack.add_module(PrintLn);

    rack.add_wire(Wire { from: (n1, 0), to: (n2, 0) });


}
