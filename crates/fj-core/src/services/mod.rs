//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;
mod validation;

use crate::objects::{Object, ObjectSet, Objects, WithHandle};

pub use self::{
    objects::{InsertObject, Operation},
    service::{Service, State},
    validation::{Validation, ValidationCommand, ValidationEvent},
};

/// The kernel services
pub struct Services {
    /// The objects service
    ///
    /// Allows for inserting objects into a store after they were created.
    pub objects: Service<Objects>,

    /// The validation service
    ///
    /// Validates objects that are inserted using the objects service.
    pub validation: Service<Validation>,
}

impl Services {
    /// Construct an instance of `Services`
    pub fn new() -> Self {
        let objects = Service::<Objects>::default();
        let validation = Service::default();

        Self {
            objects,
            validation,
        }
    }

    /// Insert an object into the stores
    pub fn insert_object(&mut self, object: Object<WithHandle>) {
        let mut object_events = Vec::new();
        self.objects
            .execute(Operation::InsertObject { object }, &mut object_events);

        for object_event in object_events {
            let command = ValidationCommand::ValidateObject {
                object: object_event.object.into(),
            };
            self.validation.execute(command, &mut Vec::new());
        }
    }

    /// Validate the provided objects and forget all other validation errors
    pub fn only_validate(&mut self, objects: impl Into<ObjectSet>) {
        let objects = objects.into();

        let mut events = Vec::new();
        self.validation
            .execute(ValidationCommand::OnlyValidate { objects }, &mut events);
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
