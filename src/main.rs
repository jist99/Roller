use std::io;
use std::io::Write;
use rand::Rng;
use std::process;

#[derive(Debug)]
enum Operator {
    Roll,
    Add,
    Sub,
    Roll1,
}

impl Operator {
    fn val(&self) -> i32 {
        match self {
            Operator::Roll => 2,
            Operator::Add => 1,
            Operator::Sub => 1,
            _ => 0,
        }
    }
}

#[derive(Debug)]
enum Opop { //Operator/Operand
    Operator(Operator),
    Operand(i32),
}

struct Shunter {
    op_stack: Vec<Operator>,
    out_q: Vec<Opop>,
    operations: i32,
}

impl Shunter {
    fn new() -> Shunter {
        Shunter{op_stack: Vec::<Operator>::new(), out_q: Vec::<Opop>::new(), operations: 0}
    }

    fn push(&mut self, input: Opop) {
        self.operations += 1;

        match input {
            Opop::Operand(_) => self.out_q.push(input),
            Opop::Operator(v) => {
                while self.op_stack.len() >= 1 && self.op_stack[self.op_stack.len() - 1].val() > v.val() {
                    let o = self.op_stack.pop().unwrap();
                    self.out_q.push(Opop::Operator(o));
                }

                self.op_stack.push(v);
            }
        };
    }

    fn clear_op(&mut self) {
        for _i in 0..self.op_stack.len() {
            let o = self.op_stack.pop().unwrap();
            self.out_q.push(Opop::Operator(o));
        }
    }

    fn clear(&mut self) {
        self.op_stack.clear();
        self.out_q.clear();
        self.operations = 0;
    }
}

//Helpers
fn read_num(pos: &mut usize, chars: &Vec<char>) -> i32 {
    let mut num = String::new();
    while chars[*pos].is_numeric() {
        num.push(chars[*pos]);
        *pos += 1;
    }

    num.parse().unwrap()
}

fn is_str(pos: &mut usize, chars: &Vec<char>, s: &str) -> bool {
    let mut tpos = 0usize;
    let s: Vec<char> = s.chars().collect();

    for i in *pos..chars.len() {
        if chars[i] != s[tpos] {
            return false
        }
        tpos += 1;
        
        if tpos >= s.len() {
            break;
        }
    }

    *pos += tpos-1;
    true
}


fn main() {
    let mut shunt = Shunter::new();
    let mut comp_stack = Vec::<i32>::new();

    let mut ans = 0;

    let running = true;

    'a: while running {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        
        let mut consecutive = 0;
        let mut cans = 0;
        //let parts = command.split_whitespace();

        shunt.clear();
        comp_stack.clear();

        //push things through shunter (lex)
        let chars: Vec<char> = command.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            match c {
                c if c.is_numeric() => {
                    if consecutive == 1 {
                        shunt.push(Opop::Operator(Operator::Roll));
                        shunt.push(Opop::Operand(read_num(&mut i, &chars)));

                        consecutive = 0;
                    }
                    else {
                        shunt.out_q.push(Opop::Operand(read_num(&mut i, &chars)));
                        consecutive += 1;
                    }
                },
                '+' => {
                    shunt.push(Opop::Operator(Operator::Add));
                    consecutive = 0;
                    i+=1;
                },
                '-' => {
                    shunt.push(Opop::Operator(Operator::Sub));
                    consecutive = 0;
                    i+=1;
                },
                _c if is_str(&mut i, &chars, "exit") => process::exit(0),
                _ => i+=1, 
            }
        }

        shunt.clear_op();
        //println!("{:?}", shunt.out_q);

        //Check if a single dice is being rolled
        if shunt.operations == 0 {
            shunt.out_q.push(Opop::Operator(Operator::Roll1));
        }

        for out in &mut shunt.out_q {
            match out {
                Opop::Operand(v) => comp_stack.push(*v),

                Opop::Operator(op) => {
                    match op {
                        Operator::Roll1 => {
                            if comp_stack.len() >= 1 {
                                let n = comp_stack.pop().unwrap();

                                if n <= 0 {
                                    println!("Cannot roll zero sided dice!");
                                    continue 'a;
                                }

                                let r = rand::thread_rng().gen_range(1..n + 1);
                                println!("1d{} | {} |", n, r);

                                cans = r;
                            }
                            else { //when no input, roll a d20
                                println!("1d20 | {} |", rand::thread_rng().gen_range(1..21));
                            }
                        }

                        Operator::Roll => {
                            if comp_stack.len() >= 2 {
                                let mut total = 0;
                                let r = comp_stack.pop().unwrap();
                                let n = comp_stack.pop().unwrap();

                                if r <= 0 {
                                    println!("Cannot roll zero sided dice!");
                                    continue 'a;
                                }
                                
                                print!("{}d{:<2} ", n, r);

                                for _i in 0..n as usize {
                                    let roll = rand::thread_rng().gen_range(1..r+1);

                                    total += roll;
                                    print!("| {:>2} ", roll);
                                }

                                print!("| = {}", total);
                                println!();

                                comp_stack.push(total);
                                cans = total;
                            }
                            else {
                                println!("No data!");
                            }
                        }

                        Operator::Add => {
                            if comp_stack.len() >= 2 {
                                println!("{}", comp_stack.pop().unwrap() + comp_stack.pop().unwrap());
                            }
                            else if comp_stack.len() == 1 {
                                println!("{}", ans + comp_stack.pop().unwrap());
                            }
                        }

                        Operator::Sub => {
                            if comp_stack.len() >= 2 {
                                let a = comp_stack.pop().unwrap();
                                let b = comp_stack.pop().unwrap();
                                println!("{}", b - a);
                                cans = b - a;
                            }
                            else if comp_stack.len() == 1 {
                                let a = comp_stack.pop().unwrap();
                                println!("{}", ans - a);
                                cans = ans-a;
                            }
                        }

                        //_ => println!("WIP"),
                    }
                }
            }
        }

        ans = cans;
    }
}
