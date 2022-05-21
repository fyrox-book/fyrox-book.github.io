# Animation (WIP)

 Animation blending state machine.

 Machine is used to blend multiple animation as well as perform automatic "smooth transition
 between states. Let have a quick look at simple machine graph:

 ```text
                                                  +-------------+
                                                  |  Idle Anim  |
                                                  +------+------+
                                                         |
           Walk Weight                                   |
 +-----------+      +-------+           Walk->Idle Rule  |
 | Walk Anim +------+       |                            |
 +-----------+      |       |      +-------+         +---+---+
                    | Blend |      |       +-------->+       |
                    |       +------+ Walk  |         |  Idle |
 +-----------+      |       |      |       +<--------+       |
 | Aim Anim  +------+       |      +--+----+         +---+---+
 +-----------+      +-------+         |                  ^
           Aim Weight                 | Idle->Walk Rule  |
                                      |                  |
                       Walk->Run Rule |    +---------+   | Run->Idle Rule
                                      |    |         |   |
                                      +--->+   Run   +---+
                                           |         |
                                           +----+----+
                                                |
                                                |
                                         +------+------+
                                         |  Run Anim   |
                                         +-------------+
 ```

 Here we have Walk, Idle, Run states which uses different sources of poses:
 - Walk - is most complicated here - it uses result of blending between
   Aim and Walk animations with different weights. This is useful if your
   character can only walk or can walk *and* aim at the same time. Desired pose
   determined by Walk Weight and Aim Weight parameters combination.
 - Run and idle both directly uses animation as pose source.

 There are four transitions between three states each with its own rule. Rule
 is just Rule parameter which can have boolean value that indicates that transition
 should be activated.

 Example:

 ```rust,no_run
 # extern crate fyrox;
 use fyrox::{
     animation::machine::{
         Machine, State, Transition, PoseNode, node::blend::BlendPose,
         Parameter, PlayAnimation, PoseWeight, node::blend::BlendAnimations
     },
     core::pool::Handle
 };

 // Assume that these are correct handles.
 let idle_animation = Handle::default();
 let walk_animation = Handle::default();
 let aim_animation = Handle::default();
 let model_root = Handle::default();

 let mut machine = Machine::new(model_root);

 let aim = machine.add_node(PoseNode::PlayAnimation(PlayAnimation::new(aim_animation)));
 let walk = machine.add_node(PoseNode::PlayAnimation(PlayAnimation::new(walk_animation)));

 // Blend two animations together
 let blend_aim_walk = machine.add_node(PoseNode::BlendAnimations(
     BlendAnimations::new(vec![
         BlendPose::new(PoseWeight::Constant(0.75), aim),
         BlendPose::new(PoseWeight::Constant(0.25), walk)
     ])
 ));

 let walk_state = machine.add_state(State::new("Walk", blend_aim_walk));

 let idle = machine.add_node(PoseNode::PlayAnimation(PlayAnimation::new(idle_animation)));
 let idle_state = machine.add_state(State::new("Idle", idle));

 machine.add_transition(Transition::new("Walk->Idle", walk_state, idle_state, 1.0, "WalkToIdle"));
 machine.add_transition(Transition::new("Idle->Walk", idle_state, walk_state, 1.0, "IdleToWalk"));

 ```

 You can use multiple machines to animation single model - for example one machine can be for
 locomotion and other is for combat. This means that locomotion machine will take control over
 lower body and combat machine will control upper body.