use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

pub struct Program{
    inst : [u8 ; 8],  //.to_be_bytes(); from_be_bytes();
}

pub struct Interpreter{
    []
}

// operators:
// 00 -> add
// 01 -> subs
// 10 -> mult
// 11 -> cos
impl Interpreter{
pub fn decode_inst(inst : u8, Reg : [f32; 4], state : State){
        let op=(inst & 0xC0) >> 6;
        let aRa=(inst & 0x30) >> 4;
        let aRIb=(inst & 0x0C) >> 2;
        let aRIc=(inst & 0x03) >> 0;
        let IorRb=RIb >>1;//true is Input, false is Register
        let IorRc=RIc >>1;//true is Input, false is Register

        Rb=Reg[0+RIb[0]];
        //If it is an input...
        if IorRb==true{
            Rb=state[2+RIb[0]];
        }

        Rc=Reg[0+RIc[0]];
        //If it is an input...
        if IorRc==true{
            Rc=state[2+RIb[0]];
        }

        match op{
            // add
            0b00 => Reg[aRA]=Rb+Rc;
            // subs
            0b01 => Reg[aRA]=Rb-Rc;
            // mult
            0b10 => Reg[aRA]=Rb*Rc;
            // b*cos(c)
            0b11 => Reg[aRa]=Rb*Rc.cos();
        }
    }
}

impl Program{
    pub fn action(&self,state : State) -> f32{
        
    }
}