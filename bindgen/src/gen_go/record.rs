/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use uniffi_bindgen::{backend::Literal, interface::Type, ComponentInterface};

use super::CodeType;

#[derive(Debug)]
pub struct RecordCodeType {
    name: String,
    module_path: String,
}

impl RecordCodeType {
    pub fn new(name: String, module_path: String) -> Self {
        Self { name, module_path }
    }
}

impl CodeType for RecordCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        let class_name = super::GoCodeOracle.class_name(&self.name);
        
        // Check if this is an external type
        let type_ref = Type::Record {
            module_path: self.module_path.clone(),
            name: self.name.clone(),
        };
        
        if ci.is_external(&type_ref) {
            // Get the namespace for the external type
            if let Ok(namespace) = ci.namespace_for_module_path(&self.module_path) {
                return format!("{}.{}", namespace, class_name);
            }
        }
        
        class_name
    }

    fn canonical_name(&self) -> String {
        super::GoCodeOracle.class_name(&self.name)
    }

    fn literal(&self, _literal: &Literal, _ci: &ComponentInterface) -> String {
        unreachable!();
    }

    fn ffi_converter_name(&self) -> String {
        let name = format!("FfiConverter{}", self.canonical_name());
        
        // For external types, we need to qualify with the namespace
        // We don't have access to ComponentInterface here, but we can determine externality
        // by comparing module paths. For now, just return the unqualified name since
        // the template system will handle the qualification through type_label
        name
    }

    fn ffi_destroyer_name(&self) -> String {
        let name = format!("FfiDestroyer{}", self.canonical_name());
        name
    }
}
