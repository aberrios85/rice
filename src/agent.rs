//use crate::candidate::Candidate;

#[derive(Debug)]
pub enum IceState {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Copy, Clone)]
pub enum Role {
    Controlled,
    Controlling,
}

pub struct Agent {
    state: IceState,
    role: Role,
    //candidates: Vec<Candidate>,
}

pub fn set_icestate(mut agent: Agent, state: IceState) {
    agent.state = state;
}
