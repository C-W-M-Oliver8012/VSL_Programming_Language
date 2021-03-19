use std::io::Write;

const POP: i64 = 1;

const I_CONSTANT: i64 = 2;
const I_ADD: i64 = 3;
const I_SUB: i64 = 4;
const I_MUL: i64 = 5;
const I_DIV: i64 = 6;
const I_EQUAL: i64 = 7;
const I_LESS: i64 = 8;
const I_GREATER: i64 = 9;
const I_NOT_EQUAL: i64 = 10;
const I_LESS_EQUAL: i64 = 11;
const I_GREATER_EQUAL: i64 = 12;
const I_AND: i64 = 13;
const I_OR: i64 = 14;
const I_LOAD: i64 = 15;
const I_STORE: i64 = 16;

const F_CONSTANT: i64 = 17;
const F_ADD: i64 = 18;
const F_SUB: i64 = 19;
const F_MUL: i64 = 20;
const F_DIV: i64 = 21;
const F_EQUAL: i64 = 22;
const F_LESS: i64 = 23;
const F_GREATER: i64 = 24;
const F_NOT_EQUAL: i64 = 25;
const F_LESS_EQUAL: i64 = 26;
const F_GREATER_EQUAL: i64 = 27;
const F_AND: i64 = 28;
const F_OR: i64 = 29;
const F_LOAD: i64 = 30;
const F_STORE: i64 = 31;

const S_CONSTANT: i64 = 32;
const S_ADD: i64 = 33;
const S_LOAD: i64 = 34;
const S_STORE: i64 = 35;
//const S_JUMP_EQUAL: i64 = 36;
//const S_JUMP_NOT_EQUAL: i64 = 37;

const JUMP_IF_FALSE: i64 = 38;
const JUMP: i64 = 39;

const CALL: i64 = 40;
const RETURN_VAL: i64 = 41;
const RETURN_NON_VAL: i64 = 42;
const ARG_LOAD: i64 = 43;
const ARG_STORE: i64 = 44;

const HALT: i64 = 45;
const I_PRINT: i64 = 46;
const F_PRINT: i64 = 47;
const S_PRINT: i64 = 48;

pub struct VM {
    string_constants: Vec<String>,
    string_data: Vec<String>,
    stack: Vec<i64>,
    code: Vec<i64>,
    ip: usize,
    fp: usize,
    sp: usize,
    debug: bool,
    pub halt: bool,
}

impl VM {
    pub fn new(program: Vec<i64>, debug: bool) -> VM {
        let mut vm = VM {
            string_constants: Vec::new(),
            string_data: Vec::new(),
            stack: Vec::new(),
            code: Vec::new(),
            ip: 0,
            fp: 0,
            sp: 0,
            debug: debug,
            halt: false,
        };
        
        for chunck in program {
            vm.code.push(chunck);
        }
        vm
    }

    pub fn execute(&mut self) {
        while self.halt == false {
            let opcode = self.code[self.ip];
            self.ip += 1;

            match opcode {
                POP => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "pop", self.code[self.ip]);
                    }
                    let index = self.code[self.ip] as usize + self.fp;
                    self.stack.remove(index);
                    self.sp -= 1;
                    self.ip += 1;
                },
                I_CONSTANT => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "i_constant", self.code[self.ip]);
                    }
                    self.stack.push(self.code[self.ip]);
                    self.ip += 1;
                    self.sp += 1;
                },
                I_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_add");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_add(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_SUB => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_sub");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_sub(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_MUL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_mul");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_mul(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_DIV => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_div");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_div(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a == b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_LESS => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_less");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a < b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_GREATER => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_greater");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a > b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_NOT_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_not_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_LESS_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_less_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a <= b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_GREATER_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_greater_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a >= b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_OR => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_or");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != 0 || b != 0 {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_AND => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_and");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != 0 && b != 0 {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "i_load", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp;
                    self.stack.push(self.stack[index]);
                    self.ip += 1;
                    self.sp += 1;
                },
                I_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "i_store", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp;
                    let data = self.stack[self.sp - 1];
                    if index < self.stack.len() - 1 {
                        self.stack[index] = data;
                        self.stack.pop();
                        self.sp -= 1;
                    }
                    self.ip += 1;
                },
                F_CONSTANT => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "f_constant", f64::from_be_bytes(self.code[self.ip].to_be_bytes()));
                    }
                    self.stack.push(self.code[self.ip]);
                    self.ip += 1;
                    self.sp += 1;
                },
                F_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_add");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a + b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_SUB => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_sub");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a - b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_MUL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_mul");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a * b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_DIV => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_div");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a / b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a == b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_LESS => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_less");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a < b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_GREATER => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_greater");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a > b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_NOT_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_not_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a != b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_LESS_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_less_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a <= b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_GREATER_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_greater_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a >= b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_OR => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_or");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a != 0.0 || b != 0.0 {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_AND => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_and");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a != 0.0 && b != 0.0 {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                }
                F_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "f_load", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp;
                    self.stack.push(self.stack[index]);
                    
                    self.ip += 1;
                    self.sp += 1;
                },
                F_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "f_store", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp;
                    let data = self.stack[self.sp - 1];
                    if index < self.stack.len() - 1 {
                        self.stack[index] = data;
                        self.stack.pop();
                        self.sp -= 1;
                    }
                    self.ip += 1;
                },
                S_CONSTANT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_constant");
                    }
                    let mut string: String = String::new();
                    while self.code[self.ip] != 0 && self.code[self.ip] < 128 {
                        string.push(self.code[self.ip] as u8 as char);
                        self.ip += 1;
                    }
                    self.string_constants.push(string);
                    self.ip += 1;

                    self.stack.push(self.string_constants.len() as i64 - 1);
                    self.sp += 1;
                },
                S_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_add");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(a);
                    self.sp -= 1;

                    self.string_constants[a as usize] = self.string_constants[a as usize].clone() + &self.string_constants[b as usize].clone();
                },
                S_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "s_load", self.code[self.ip]);
                    }
                    let mem_location = self.code[self.ip] as usize;
                    self.string_constants.push(self.string_data[mem_location].clone());
                    self.stack.push(self.string_constants.len() as i64 - 1);
                    self.sp += 1;
                    self.ip += 1;
                },
                S_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "s_store", self.code[self.ip]);
                    }

                    let index: i64 = self.code[self.ip];
                    if index == self.string_data.len() as i64 {
                        self.string_data.push(self.string_constants[self.stack[self.sp - 1] as usize].clone());
                    } else if index < self.string_data.len() as i64 {
                        self.string_data[index as usize] = self.string_constants[self.stack[self.sp - 1] as usize].clone();
                    } else if index > self.string_data.len() as i64 {
                        self.string_data.resize(index as usize + 1, String::new());
                        self.string_data[index as usize] = self.string_constants[self.stack[self.sp - 1] as usize].clone();
                    }

                    self.stack.pop();
                    self.sp -= 1;
                    self.ip += 1;
                },
                JUMP_IF_FALSE => {
                    if self.debug {
                        println!("{}: {} {} {}", self.ip - 1, "jump_if_false", self.stack[self.sp - 1], self.code[self.ip]);
                    }
                    let location = self.code[self.ip] as usize;
                    let boolean_value = self.stack[self.sp - 1];
                    self.stack.pop();
                    if boolean_value == 0 {
                        self.ip = location;
                    } else {
                        self.ip += 1;
                    }
                    self.sp -= 1;
                },
                JUMP => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "jump", self.code[self.ip]);
                    }
                    let location = self.code[self.ip] as usize;
                    self.ip = location;
                },
                CALL => {
                    if self.debug {
                        println!("{}: {} {} {}", self.ip - 1, "call", self.code[self.ip], self.code[self.ip + 1]);
                    }
                    let address: i64 = self.code[self.ip];
                    let nargs: i64 = self.code[self.ip + 1];
                    let return_address: i64 = self.ip as i64 + 2;

                    self.stack.push(return_address);
                    self.stack.push(nargs);

                    self.stack.push(self.fp as i64);
                    self.fp = self.sp;

                    self.sp += 3;
                    self.ip = address as usize;
                },
                RETURN_VAL => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "return", self.stack[self.sp - 1]);
                    }
                    let return_value: i64 = self.stack[self.sp - 1];
                    let fp: usize = self.stack[self.sp - 2] as usize;
                    let nargs: usize = self.stack[self.sp - 3] as usize;
                    let return_address: usize = self.stack[self.sp - 4] as usize;
                    self.sp = self.fp - nargs;
                    self.fp = fp;
                    self.ip = return_address;

                    while self.sp != self.stack.len() {
                        self.stack.pop();
                    }
                    self.stack.push(return_value);
                    self.sp += 1;
                },
                RETURN_NON_VAL => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "return", self.stack[self.sp - 1]);
                    }
                    let fp: usize = self.stack[self.sp - 1] as usize;
                    let nargs: usize = self.stack[self.sp - 2] as usize;
                    let return_address: usize = self.stack[self.sp - 3] as usize;
                    self.sp = self.fp - nargs;
                    self.fp = fp;
                    self.ip = return_address;

                    while self.sp != self.stack.len() {
                        self.stack.pop();
                    }
                },
                ARG_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "arg_load", self.code[self.ip]);
                    }
                    let offset: i64 = self.code[self.ip];
                    let num_args: i64 = self.stack[self.fp + 1];
                    let value: i64 = self.stack[self.fp - num_args as usize + offset as usize];
                    self.stack.push(value);
                    self.sp += 1;
                    self.ip += 1;
                },
                ARG_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "arg_store", self.code[self.ip]);
                    }
                    let offset: i64 = self.code[self.ip];
                    let num_args: i64 = self.stack[self.fp + 1];
                    let value: i64 = self.stack[self.sp - 1];
                    self.stack[self.fp - num_args as usize + offset as usize] = value;
                    self.stack.pop();
                    self.sp -= 1;
                    self.ip += 1;
                },
                HALT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "halt");
                    }
                    self.halt = true;
                },
                I_PRINT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_print");
                    }
                    print!("{}", self.stack[self.sp - 1]);
                    self.stack.pop();
                    self.sp -= 1;
                    std::io::stdout().flush().expect("Failed to flush stdout.");
                },
                F_PRINT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_print");
                    }
                    let f_bytes = self.stack[self.sp - 1].to_be_bytes();
                    print!("{}", f64::from_be_bytes(f_bytes));
                    self.stack.pop();
                    self.sp -= 1;
                    std::io::stdout().flush().expect("Failed to flush stdout.");
                },
                S_PRINT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_print");
                    }
                    let location = self.stack[self.sp - 1] as usize;
                    self.stack.pop();
                    self.sp -= 1;
                    print!("{}", self.string_constants[location]);
                    std::io::stdout().flush().expect("Failed to flush stdout.");
                }
                _ => panic!("Bad Opcode: {}", opcode),
            }
            if self.debug {
                println!("{:?} {} {}", self.stack, self.sp, self.fp);
                println!("{:?}\n", self.string_constants);
            }
        }
    }
}