#![allow(missing_docs)] // TODO

use crate::scene::variable::InheritError;
use crate::{
    core::{
        inspect::{Inspect, PropertyInfo},
        pool::Handle,
        visitor::prelude::*,
    },
    scene::{
        base::{Base, BaseBuilder},
        graph::Graph,
        node::Node,
    },
};
use fyrox_core::math::aabb::AxisAlignedBoundingBox;
use std::ops::{Deref, DerefMut};

#[derive(Visit, Inspect, Default, Debug)]
pub struct Listener {
    base: Base,
}

impl Deref for Listener {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Listener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Listener {
    pub fn raw_copy(&self) -> Self {
        Self {
            base: self.base.raw_copy(),
        }
    }

    // Prefab inheritance resolving.
    pub(crate) fn inherit(&mut self, parent: &Node) -> Result<(), InheritError> {
        self.base.inherit_properties(parent)
    }

    pub(crate) fn reset_inheritable_properties(&mut self) {
        self.base.reset_inheritable_properties();
    }

    pub fn local_bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

pub struct ListenerBuilder {
    base_builder: BaseBuilder,
}

impl ListenerBuilder {
    pub fn new(base_builder: BaseBuilder) -> Self {
        Self { base_builder }
    }

    pub fn build_listener(self) -> Listener {
        Listener {
            base: self.base_builder.build_base(),
        }
    }

    pub fn build_node(self) -> Node {
        Node::Listener(self.build_listener())
    }

    pub fn build(self, graph: &mut Graph) -> Handle<Node> {
        graph.add_node(self.build_node())
    }
}

#[cfg(test)]
mod test {
    use crate::scene::{
        base::{test::check_inheritable_properties_equality, BaseBuilder},
        node::Node,
        sound::listener::ListenerBuilder,
    };

    #[test]
    fn test_listener_inheritance() {
        let parent = ListenerBuilder::new(BaseBuilder::new()).build_node();

        let mut child = ListenerBuilder::new(BaseBuilder::new()).build_listener();

        child.inherit(&parent).unwrap();

        if let Node::Listener(parent) = parent {
            check_inheritable_properties_equality(&child.base, &parent.base);
        } else {
            unreachable!()
        }
    }
}
