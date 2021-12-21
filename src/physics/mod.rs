use rapier2d::prelude::*;

pub struct Physics {
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
    ball_handle: RigidBodyHandle,
}

impl Physics {
    pub fn new() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let ground = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(ground);

        let ball_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let ball_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();

        let ball_handle = rigid_body_set.insert(ball_body);
        collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

        Self {
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            collider_set,
            rigid_body_set,
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            ball_handle,
        }
    }

    pub fn update(&mut self) {
        self.physics_pipeline.step(
            &vector![0.0, -9.81],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        );

        let ball_body = &self.rigid_body_set[self.ball_handle];
        println!("Ball altitude: {}", ball_body.translation().y);
    }
}
