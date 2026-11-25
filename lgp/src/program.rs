use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

// Alias Instruction
pub type Instruction = u8;


// struct Program
// Instructions have the format op Ra Rb Rc with op in [b+c, b-c, a*b, a*cos(b)]
pub struct Program{
    insts : [Instruction ; 8];  //.to_be_bytes(); from_be_bytes();
}

impl Program{
    fn new_rd() -> (Self) {// Constructor (random 8 instructions)
        let i=0;
        let insts;
        for i in 0..8 {
            let random_u8: u8 = rand::random();
            insts[i] = random_u8;// An instruction is generated as a random 8bits number
        }
        Program{insts : insts}
    }
    pub fn action(&self,state : State) -> f32{
        
    }
}

//$$$$$$$$$$$$$$$$$$$$$$$$$$

// struct Interpreter
// 
pub struct Interpreter{
    
    counters : [u32 ; 8]; // Counts the number of times the instructions were executed. To avoid loops.
    PC : u8;// Nb of the instruction that is currently active
}

// Operators:
// 00 -> add
// 01 -> subs
// 10 -> mult
// 11 -> cos
impl Interpreter{
    let regs : [Reg ; 4];
    // Decode and execute a single instrcution
    pub fn execute_inst(inst : u8, Reg : [f32; 4], state : State) -> (u8){
        let op=(inst & 0xC0) >> 6;
        let aRa=(inst & 0x30) >> 4;
        let aRIb=(inst & 0x0C) >> 2;
        let aRIc=(inst & 0x03) >> 0;
        let IorRb=RIb >> 1;         //true is Input, false is Register
        let IorRc=RIc >> 1;         //true is Input, false is Register
        let offset=(inst & 0x30) >> 4;// Offset used for jumps

        Rb=Reg[0+RIb[0]];
        //If it is an input...
        if IorRb==true{
            Rb=state[2+RIb[0]];
        }

        Rc=Reg[2+RIc[0]];
        //If it is an input...
        if IorRc==true{
            Rc=state[0+RIb[0]];
        }

        match op{
            // add
            0b00 => Reg[aRA]=Rb+Rc;
            return PC++;
            // subs
            0b01 => Reg[aRA]=Rb-Rc;
            return PC++;
            // mult
            0b10 => Reg[aRA]=Rb*Rc;
            return PC++;
            // b*cos(c)
            //0b11 => Reg[aRa]=Rb*Rc.cos();
            return PC++;
            // b*cos(c)
            0b11 => Reg[aRa]=Rb*Rc.cos();
            return PC-=;
        }
    }

    //
    pub fn execute_prog(state : State) -> (bool){
        self.PC=0
        while (self.PC!=8 or self.counters[self.PC]<100){// Loops greater than 100 are considered as infinite loops and thus we break the loop
            self.counters[self.PC]+=1;
            // If JMP instr, PC=new_PC, else new_PC will be PC++
            self.PC=self.execute_instr(self.insts[self.PC], self.regs, self.state, self.PC);
        }
        // false means nothing weird happened (the whole program got executed)
        // true means an infinite loop was the reason teh program stopped
        if self.PC==8{ return false; } else{ return true }
    }
}
