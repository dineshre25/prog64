use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

pub struct Program{
    inst : [u8 ; 8],  //.to_be_bytes(); from_be_bytes();
}

fn decode_inst(inst : u8, Reg : [f32; 4], state : State){
    let a =
}

impl Program{
    pub fn action(&self,state : State) -> f32{
        
    }
}