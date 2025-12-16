use oxc_syntax::{node::NodeId, scope::ScopeId};

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub scope_id: Option<ScopeId>,
    unreachable: bool,
}

impl BasicBlock {
    pub(crate) fn new() -> Self {
        BasicBlock { instructions: Vec::new(), scope_id: None,unreachable: false }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn set_scope_id(&mut self, scope_id: Option<ScopeId>) {
        self.scope_id = scope_id;
    }

    #[inline]
    pub fn is_unreachable(&self) -> bool {
        self.unreachable
    }

    #[inline]
    pub fn mark_as_unreachable(&mut self) {
        self.unreachable = true;
    }

    #[inline]
    pub fn mark_as_reachable(&mut self) {
        self.unreachable = false;
    }

    
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub node_id: Option<NodeId>,
    pub scope_id: Option<ScopeId>,
}

impl Instruction {
    pub fn new(kind: InstructionKind, node_id: Option<NodeId>, scope_id: Option<ScopeId>) -> Self {
        Self { kind, node_id, scope_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionKind {
    Unreachable,
    Statement,
    ImplicitReturn,
    Return(ReturnInstructionKind),
    Break(LabeledInstruction),
    Continue(LabeledInstruction),
    Throw,
    Condition,
    Iteration(IterationInstructionKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnInstructionKind {
    ImplicitUndefined,
    NotImplicitUndefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabeledInstruction {
    Labeled,
    Unlabeled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationInstructionKind {
    Of(NodeId),
    In(NodeId),
}
