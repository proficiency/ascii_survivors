use bevy::prelude::*;

#[derive(Event)]
pub struct WalkEvent {
    pub direction: IVec2,
}

#[derive(Event)]
pub enum UiActionEvent {
    Navigate(IVec2),
    Submit,
    Cancel,
    Info,
}

#[derive(Event)]
pub struct InteractEvent;
