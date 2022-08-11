use crate::{Address, BlockId, OperationId, Slot};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// By product of a byte code execution
pub struct SCOutputEvent {
    /// context generated by the execution context
    pub context: EventExecutionContext,
    /// json data string
    pub data: String,
}

impl Display for SCOutputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Context: {}", self.context)?;
        writeln!(f, "Data: {}", self.data)
    }
}

/// Context of the event (not generated by the user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventExecutionContext {
    /// when was it generated
    pub slot: Slot,
    /// block id if there was a block at that slot
    pub block: Option<BlockId>,
    /// if the event was generated during a read only execution
    pub read_only: bool,
    /// index of the event in the slot
    pub index_in_slot: u64,
    /// most recent at the end
    pub call_stack: VecDeque<Address>,
    /// origin operation id
    pub origin_operation_id: Option<OperationId>,
}

impl Display for EventExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Slot: {} at index: {}", self.slot, self.index_in_slot)?;

        writeln!(
            f,
            "{}",
            if self.read_only {
                "Read only execution"
            } else {
                "On chain execution"
            }
        )?;
        if let Some(id) = self.block {
            writeln!(f, "Block id: {}", id)?;
        }
        if let Some(id) = self.origin_operation_id {
            writeln!(f, "Origin operation id: {}", id)?;
        }
        writeln!(
            f,
            "Call stack: {}",
            self.call_stack
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}