// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Class name builder utility
//!
//! Provides a type-safe way to build CSS class strings,
//! similar to the clsx library used in the React implementation.

/// Builder for constructing CSS class strings
#[derive(Default)]
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    /// Creates a new class builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a class unconditionally
    pub fn add(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Adds a class conditionally
    pub fn add_if(mut self, condition: bool, class: impl Into<String>) -> Self {
        if condition {
            self.classes.push(class.into());
        }
        self
    }

    /// Adds an optional class
    pub fn add_option(mut self, class: Option<impl Into<String>>) -> Self {
        if let Some(c) = class {
            self.classes.push(c.into());
        }
        self
    }

    /// Builds the final class string
    pub fn build(self) -> String {
        self.classes.join(" ")
    }
}

impl From<ClassBuilder> for String {
    fn from(builder: ClassBuilder) -> Self {
        builder.build()
    }
}

/// Convenience function for building classes
pub fn classes() -> ClassBuilder {
    ClassBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_builder() {
        let result = ClassBuilder::new()
            .add("base")
            .add_if(true, "active")
            .add_if(false, "disabled")
            .add_option(Some("optional"))
            .add_option(None::<String>)
            .build();

        assert_eq!(result, "base active optional");
    }

    #[test]
    fn test_empty_builder() {
        let result = ClassBuilder::new().build();
        assert_eq!(result, "");
    }

    #[test]
    fn test_classes_convenience() {
        let result = classes()
            .add("one")
            .add("two")
            .build();

        assert_eq!(result, "one two");
    }
}
