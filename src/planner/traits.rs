use crate::planner::plan::Plan;

#[doc = "Trait for planner"]
pub trait Planner {
    fn plan(&self) -> Plan;
}
