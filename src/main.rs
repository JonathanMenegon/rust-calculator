use std::io;
#[derive(PartialEq)]
enum Token{
    NUMBER(String),
    OPERATOR(char),
}

impl Token {
    fn get_value(self){

    }
}

fn main() {
    let mut expression: String = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Hello");
    
    let expression: String = expression.trim().to_string();

    let t_vec: Vec<Token> = tokenize(expression);
    let t_vec: Vec<Token> = shunt_yard_alg(t_vec);
    // for i in t_vec{
    //     println!("{:?}",i);
    // }
    let res:f64 = calc(t_vec);
    println!("Your Result is {}",res);
}

// My AMAZING implementation of the Shunting Yard Algorithm feat. Dijkstra (The Witcher one)
// This Might be the worst code, or a tie, or a fucking mess. Mostly likely all 3 at once
// I spent more time on this than ill ever admit
fn shunt_yard_alg(t_vec: Vec<Token>) -> Vec<Token>{
    let mut stack:Vec<Token> = Vec::new();
    let mut queue:Vec<Token> = Vec::new();
    for token in t_vec{
        match token {
            Token::NUMBER(_) => queue.push(token),
            operator @ Token::OPERATOR(_) => 
            {
                let temp = stack.pop();
                if temp == None{
                    stack.push(operator);
                }
                else {
                    stack.push(temp.unwrap());
                     if /*precedence(&operator) == 2*/ false{
                        stack.push(operator)
                    }
                    else {
                        loop{
                                let temp = stack.pop();
                                match temp{
                                    None => {stack.push(operator); break;},
                                
                                    op @ Some(_) => {
                                        let op = op.unwrap();
                                        if precedence(&op) < precedence(&operator){
                                            stack.push(op);
                                            stack.push(operator);
                                            break;
                                        }
                                        else {
                                            queue.push(op)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    let w = 0..stack.len();
    for i in w{
        let temp = stack.pop();
        match temp{
            None => {},
            Some(_) =>{queue.push(temp.unwrap());},
        }
    }

    queue
}



// This is Some of the Worst Code Ive Ever Written
fn tokenize (expression: String) -> Vec<Token> {
    let mut t_vec_1: Vec<char> = Vec::new();
    for c in expression.chars(){
        match c {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                t_vec_1.push(c);
            }
            '+' | '-' | '*' | '/' => {
                t_vec_1.push(c);
            }
            _ => {}
        }
    }
    let mut t_vec:Vec<Token> = Vec::new();
    let mut i = 1;
    let mut j = 0;
    while i < t_vec_1.len()+1{
        if t_vec_1[i-1].is_numeric(){
            i = i + 1;
        } 
        else {
            t_vec.push(Token::NUMBER(expression[j..i-1].to_string()));
            t_vec.push(Token::OPERATOR(t_vec_1[i-1] as char));
            j=i;
            i=i+1;
        }
    }
    t_vec.push(Token::NUMBER(expression[j..].to_string()));
    t_vec
}


fn precedence(part: &Token) -> i32{
    match part {
        &Token::NUMBER(_) => 0,
        &Token::OPERATOR('+') | &Token::OPERATOR('-') => 1,
        &Token::OPERATOR('*') | &Token::OPERATOR('/') => 2,
        _ => {-1},
    }
}

fn calc(t_vec: Vec<Token>) -> f64{
    let mut stack: Vec<f64> = Vec::new();
    for token in t_vec{
        if let Token::NUMBER(ref num) = token{
            let temp = num.parse::<f64>().unwrap();
            stack.push(temp);
        }
        match token {
            Token::OPERATOR('+') =>
            {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x+y);
            },
            Token::OPERATOR('-') =>
            {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x-y);
            },
            Token::OPERATOR('*') =>
            {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x*y);
            },
            Token::OPERATOR('/') =>
            {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y/x);
            },
            _ => {}
        }
    }
    stack[0]
}