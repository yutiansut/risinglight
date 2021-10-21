use super::*;
use crate::{binder::BoundExpr, logical_planner::LogicalProjection};

/// The physical plan of project operation.
#[derive(Debug, PartialEq, Clone)]
pub struct PhysicalProjection {
    pub project_expressions: Vec<BoundExpr>,
    pub child: Box<PhysicalPlan>,
}

impl PhysicalPlaner {
    pub fn plan_projection(
        &self,
        plan: LogicalProjection,
    ) -> Result<PhysicalPlan, PhysicalPlanError> {
        Ok(PhysicalPlan::Projection(PhysicalProjection {
            project_expressions: plan.project_expressions,
            child: Box::new(self.plan(*plan.child)?),
        }))
    }
}