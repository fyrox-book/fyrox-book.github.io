use fyrox::{
    core::visitor::prelude::*,
    dispatch_behavior_variants,
    utils::behavior::{leaf, sequence, Behavior, BehaviorResult, BehaviorTree, Status},
};

// ANCHOR: beh_tree
struct Environment {
    // > 0 - door in front of
    // < 0 - door is behind
    distance_to_door: f32,
    door_opened: bool,
    done: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            distance_to_door: 3.0,
            door_opened: false,
            done: false,
        }
    }
}

#[derive(Debug, PartialEq, Default, Visit, Clone)]
struct WalkAction;

impl Behavior<'_> for WalkAction {
    type Context = Environment;

    fn tick(&mut self, context: &mut Self::Context) -> BehaviorResult {
        if context.distance_to_door <= 0.0 {
            Ok(Status::Success)
        } else {
            context.distance_to_door -= 0.1;
            println!(
                "Approaching door, remaining distance: {}",
                context.distance_to_door
            );
            Ok(Status::Running)
        }
    }
}

#[derive(Debug, PartialEq, Default, Visit, Clone)]
struct OpenDoorAction;

impl Behavior<'_> for OpenDoorAction {
    type Context = Environment;

    fn tick(&mut self, context: &mut Self::Context) -> BehaviorResult {
        if !context.door_opened {
            context.door_opened = true;
            println!("Door was opened!");
        }
        Ok(Status::Success)
    }
}

#[derive(Debug, PartialEq, Default, Visit, Clone)]
struct StepThroughAction;

impl Behavior<'_> for StepThroughAction {
    type Context = Environment;

    fn tick(&mut self, context: &mut Self::Context) -> BehaviorResult {
        if context.distance_to_door < -1.0 {
            Ok(Status::Success)
        } else {
            context.distance_to_door -= 0.1;
            println!(
                "Stepping through doorway, remaining distance: {}",
                -1.0 - context.distance_to_door
            );
            Ok(Status::Running)
        }
    }
}

#[derive(Debug, PartialEq, Default, Visit, Clone)]
struct CloseDoorAction;

impl Behavior<'_> for CloseDoorAction {
    type Context = Environment;

    fn tick(&mut self, context: &mut Self::Context) -> BehaviorResult {
        if context.door_opened {
            context.door_opened = false;
            context.done = true;
            println!("Door was closed");
        }
        Ok(Status::Success)
    }
}

#[derive(Debug, PartialEq, Visit, Clone)]
enum BotAction {
    Walk(WalkAction),
    OpenDoor(OpenDoorAction),
    StepThrough(StepThroughAction),
    CloseDoor(CloseDoorAction),
}

impl Default for BotAction {
    fn default() -> Self {
        Self::Walk(Default::default())
    }
}

dispatch_behavior_variants!(
    BotAction,
    Environment,
    Walk,
    OpenDoor,
    StepThrough,
    CloseDoor
);

fn create_tree() -> BehaviorTree<BotAction> {
    let mut tree = BehaviorTree::new();

    let entry = sequence(
        [
            leaf(BotAction::Walk(WalkAction), &mut tree),
            leaf(BotAction::OpenDoor(OpenDoorAction), &mut tree),
            leaf(BotAction::StepThrough(StepThroughAction), &mut tree),
            leaf(BotAction::CloseDoor(CloseDoorAction), &mut tree),
        ],
        &mut tree,
    );

    tree.set_entry_node(entry);

    tree
}

fn test_behavior() {
    let tree = create_tree();

    let mut ctx = Environment::default();

    while !ctx.done {
        tree.tick(&mut ctx).unwrap();
    }
}

// ANCHOR_END: beh_tree
