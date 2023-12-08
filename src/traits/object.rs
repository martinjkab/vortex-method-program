use super::{drawable::Drawable, steppable::Steppable};

pub trait Object: Drawable + Steppable {}

impl<T> Object for T where T: Drawable + Steppable {}
