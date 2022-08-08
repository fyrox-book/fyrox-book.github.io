# Animation Blending

Animation blending is a powerful feature that allows you to mix multiple animations into one. Each animation
is mixed with a various weights which in sum gives 1.0 (100%). By having opposite coefficients (k1 = 0 -> 1, k2 = 1 -> 0)
changing in time it is possible to create transition effect. 

Handling transitions with all the coefficients is a routine job, the engine can handle it for you giving you some nice
features:

- Multiple states with smooth transitions between them
- Ability to blend multiple animations in one and use it as pose source for blending
- Ability to specify a set of variables that will be used as blending coefficients and transition rules.

All these features consolidated in so-called animation blending state machine (ABSM). Machine is used to blend multiple 
animation as well as perform automatic "smooth" transition between states. Let's have a quick look at a simple machine
graph:

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

Here we have Walk, Idle and Run states which use different sources of poses:
- Walk - is the most complicated here - it uses result of blending between `Aim` and `Walk` animations with different 
weights. This is useful if your character can only walk or can walk *and* aim at the same time. Desired pose determined
by Walk Weight and Aim Weight parameters combination.
- Run and idle both directly use animation as pose source.

There are four transitions between three states each with its own rule. Rule is just a boolean parameter that indicates 
that transition should be activated. Let's look at the code example of the above state graph:

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

As you can see, everything is quite straightforward. Even such simple state machine requires quite a lot of code, which
can be removed by using ABSM editor. Read the next chapter to learn about it.

## Multiple ABSM per model

You can use multiple machines to animate single model - for example one machine can be used for locomotion and other for 
combat. This means that locomotion machine will take control over lower body and combat machine will control upper body.