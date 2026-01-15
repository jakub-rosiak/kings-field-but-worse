use crate::states::actions::Actions;

pub trait InputMap<I> {
    fn action_pressed(&self, actions: Actions, input: &I) -> bool;
    fn action_just_pressed(&self, actions: Actions, input: &I) -> bool;
    fn action_just_released(&self, actions: Actions, input: &I) -> bool;
}
