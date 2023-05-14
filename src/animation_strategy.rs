use serde::{Serialize, Deserialize};
use strum::{EnumString, EnumVariantNames};


pub enum Css {
    None,
    Keyframes,
    Transform,
    Transition,
}


pub type Push = Css;
pub type Stretch = Css;
type BaseClass = String;
type PushClass = String;
type StretchClass = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Strategy {
    None,
    Transform,
    Transition,
    ScaleAndSlide,
}

impl Strategy {
    pub fn parts(&self) -> (Stretch, Push) {
        match self {
            Self::None => (Stretch::None, Push::None),
            Self::Transform => (Stretch::Transform, Push::None),
            Self::ScaleAndSlide => (Stretch::Keyframes, Push::Keyframes),
            Self::Transition => (Stretch::Transition, Push::None),
        }
    }
}


impl Stretch {
    pub fn stretch_classes(&self) -> (BaseClass, StretchClass) {
        let vals = match self {
            Self::None => ("", ""),
            Self::Keyframes => ("", " stretch-keyframes"),
            Self::Transform => (" stretch-transform-base", " stretch-transform"),
            Self::Transition => (" stretch-transition-base", " stretch-transition"),
        };
        (vals.0.into(), vals.1.into())
    }
}


impl Push {
    pub fn push_classes(&self) -> (BaseClass, PushClass) {
        let vals = match self {
            Self::None => ("", ""),
            Self::Keyframes => ("", " push-keyframes"),
            Self::Transform => (" push-transform-base", " push-transform"),
            Self::Transition => (" push-transition-base", " push-transition"),
        };
        (vals.0.into(), vals.1.into())
    }
}


pub fn get_classes(strategy: Strategy) -> (BaseClass, StretchClass, PushClass) {
    let (stretch_approach, push_approach) = strategy.parts();
    let (no_stretch, stretch) = stretch_approach.stretch_classes();
    let (no_push, push) = push_approach.push_classes();
    
    (
        no_push.clone() + &no_stretch,
        no_push + &stretch,
        no_stretch + &push,
    )
}

