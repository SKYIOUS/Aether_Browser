// Standards compliance interface
// This module provides the interface for W3C/WHATWG specification compliance.

pub mod html5;
pub mod css3;

pub trait Specification {
    fn compliance_level(&self) -> f32;
}
