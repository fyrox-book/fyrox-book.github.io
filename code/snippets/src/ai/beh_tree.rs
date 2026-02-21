use fyrox::core::visitor::prelude::*;
use fyrox::plugin::error::GameError;
use fyrox::plugin::PluginContext;
use fyrox::utils::behavior::{inverter, leaf, selector, sequence, Behavior, Status};

#[derive(Debug, PartialEq, Visit, Clone, Default)]
pub enum Action {
    #[default]
    Unknown,
    IsDead(IsDead),
    StayDead(StayDead),
    FindTarget(FindTarget),
    ReachedTarget(IsTargetCloseBy),
    MoveToTarget(MoveToTarget),
    CanMeleeAttack(CanMeleeAttack),
    AimOnTarget(AimOnTarget),
    DoMeleeAttack(DoMeleeAttack),
    CanShootTarget(CanShootTarget),
    ShootTarget(ShootTarget),
    NeedsThreatenTarget(NeedsThreatenTarget),
    ThreatenTarget(ThreatenTarget),
}

impl<'a> Behavior<'a> for Action {
    type Context = PluginContext<'a, 'a>;

    fn tick(&mut self, context: &mut Self::Context) -> Result<Status, GameError> {
        match self {
            Action::Unknown => unreachable!(),
            Action::FindTarget(v) => v.tick(context),
            Action::ReachedTarget(v) => v.tick(context),
            Action::MoveToTarget(v) => v.tick(context),
            Action::DoMeleeAttack(v) => v.tick(context),
            Action::ShootTarget(v) => v.tick(context),
            Action::CanMeleeAttack(v) => v.tick(context),
            Action::IsDead(v) => v.tick(context),
            Action::StayDead(v) => v.tick(context),
            Action::AimOnTarget(v) => v.tick(context),
            Action::CanShootTarget(v) => v.tick(context),
            Action::NeedsThreatenTarget(v) => v.tick(context),
            Action::ThreatenTarget(v) => v.tick(context),
        }
    }
}

#[derive(Default, Debug, Visit, Clone)]
pub struct BotBehavior {
    pub tree: BehaviorTree<Action>,
}

impl BotBehavior {
    pub fn new(spine: Handle<Node>, close_combat_distance: f32) -> Self {
        let mut tree = BehaviorTree::new();
        let bt = &mut tree;

        let dead_seq = sequence([IsDead::new_action(bt), StayDead::new_action(bt)], bt);

        let threaten_seq = sequence(
            [
                leaf(Action::NeedsThreatenTarget(NeedsThreatenTarget), bt),
                leaf(AimOnTarget::new_action(spine, AimTarget::ActualTarget), bt),
                leaf(Action::ThreatenTarget(ThreatenTarget::default()), bt),
            ],
            bt,
        );

        let shooting_distance = 4.0;
        let shoot_seq = sequence(
            [
                leaf(Action::CanShootTarget(CanShootTarget), bt),
                selector(
                    [
                        sequence(
                            [
                                inverter(IsTargetCloseBy::make(shooting_distance, bt), bt),
                                leaf(
                                    AimOnTarget::new_action(spine, AimTarget::SteeringTarget),
                                    bt,
                                ),
                                leaf(
                                    Action::MoveToTarget(MoveToTarget {
                                        min_distance: shooting_distance,
                                    }),
                                    bt,
                                ),
                            ],
                            bt,
                        ),
                        leaf(AimOnTarget::new_action(spine, AimTarget::ActualTarget), bt),
                    ],
                    bt,
                ),
                leaf(Action::ShootTarget(ShootTarget), bt),
            ],
            bt,
        );

        let melee_seq = sequence(
            [
                selector(
                    [
                        sequence(
                            [
                                inverter(
                                    leaf(
                                        Action::ReachedTarget(IsTargetCloseBy {
                                            min_distance: close_combat_distance,
                                        }),
                                        bt,
                                    ),
                                    bt,
                                ),
                                leaf(
                                    AimOnTarget::new_action(spine, AimTarget::SteeringTarget),
                                    bt,
                                ),
                                leaf(
                                    Action::MoveToTarget(MoveToTarget {
                                        min_distance: close_combat_distance,
                                    }),
                                    bt,
                                ),
                            ],
                            bt,
                        ),
                        leaf(AimOnTarget::new_action(spine, AimTarget::ActualTarget), bt),
                    ],
                    bt,
                ),
                leaf(Action::CanMeleeAttack(CanMeleeAttack), bt),
                leaf(Action::DoMeleeAttack(DoMeleeAttack::default()), bt),
            ],
            bt,
        );

        let entry = selector(
            [
                dead_seq,
                sequence(
                    [
                        leaf(Action::FindTarget(FindTarget::default()), bt),
                        sequence([selector([threaten_seq, shoot_seq, melee_seq], bt)], bt),
                    ],
                    bt,
                ),
            ],
            bt,
        );

        tree.set_entry_node(entry);

        Self { tree }
    }
}
