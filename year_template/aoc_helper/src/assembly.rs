pub trait Command: Clone {
    fn run<const N: usize, T: Command>(&self, computer: &mut Computer<N, T>);
}

pub struct Computer<const N: usize, T>
where
    T: Command,
{
    pub registers: [usize; N],
    pub i: usize,
    pub program: Vec<T>,
}

impl<const N: usize, T> Computer<N, T> {
    pub fn new(program: Vec<T>) -> Self {
        Self {
            program,
            registers: [0; N],
            i: 0,
        }
    }

    pub fn run(&mut self) {
        while self.i < self.program.len() {
            let cmd = self.program[self.i].clone();
            cmd.run(self);
        }
    }
    pub fn run_debug(&mut self) {
        while self.i < self.program.len() {
            let cmd = self.program[self.i].clone();
            cmd.run(self);
            println!("registers {:?}, i: {}", &self.registers, self.i);
        }
    }
}
