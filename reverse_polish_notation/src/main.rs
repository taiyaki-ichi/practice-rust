use structopt::StructOpt;
use core::panic;
use std::fs::File;
use std::io::{BufRead,BufReader,stdin};

#[derive(StructOpt,Debug)]
#[structopt(
    name="hahaha",
    version="1.0.0",
    //auther="me",
    about="Super RPN"
)]
struct Opts{

    #[structopt(short,long)]
    verbose: bool,

    #[structopt(name="FILE")]
    formula_file: Option<String>,
}

fn main(){

    let opts=Opts::from_args();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        //BufReaderから読み込んだ方が良い
        let reader=BufReader::new(f);

        //readerは参照を渡す借用の方がいいんじゃあないのか？
       run(reader,opts.verbose);
    }else {
        let stdin=stdin();
        let reader = stdin.lock();
        run(reader,opts.verbose);
    }
}

fn run<R: BufRead>(reader: R,verbose: bool){
    let calc=RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer=calc.eval(&line);
        println!("{}",answer);
    }
}


struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self{
        Self(verbose)
    }

    pub fn eval(&self,formula: &str) -> i32  {
        let mut tokens=formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self,tokens: &mut Vec<&str>) -> i32 {
        let mut stack=Vec::new();

        while let Some(token) = tokens.pop(){
            if let Ok(x) = token.parse::<i32>(){
                stack.push(x);
            }else{
                let y=stack.pop().expect("invalid syntax");
                let x=stack.pop().expect("invalid syntax");
                let res= match token{
                    "+" => x+y,
                    "-" => x-y,
                    "/" => x/y, 
                    "%" => x%y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0{
                println!("{:?} {:?}",tokens,stack);
            }
        }

        if stack.len() == 1{
            stack[0]
        }else{
            panic!("invalid syntax");
        }
    }
}




