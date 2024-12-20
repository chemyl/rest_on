use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

//ManagingAgent — управляет группой агентов через абстрактный трейт.
//Менеджер может работать с агентами, имеющими разные реализации SpecialFunctions, не зная их внутренностей заранее.
#[derive(Debug)]
pub struct ManagingAgent {
    attribute: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}
