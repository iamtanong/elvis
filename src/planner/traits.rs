use crate::planner::plan::Plan;

pub trait Planner {
    fn plan(&self) -> Plan;
}
