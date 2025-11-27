/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use uniffi_bindgen::{backend::Literal, interface::Type, ComponentInterface};

use super::CodeType;

#[derive(Debug)]
pub struct CallbackInterfaceCodeType {
    id: String,
    module_path: String,
}

impl CallbackInterfaceCodeType {
    pub fn new(id: String, module_path: String) -> Self {
        Self { id, module_path }
    }
}

impl CodeType for CallbackInterfaceCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        let class_name = super::GoCodeOracle.class_name(&self.id);
        
        // Check if this is an external type
        let type_ref = Type::CallbackInterface {
            module_path: self.module_path.clone(),
            name: self.id.clone(),
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
        format!("CallbackInterface{}", self.id)
    }

    fn literal(&self, _literal: &Literal, _ci: &ComponentInterface) -> String {
        unreachable!();
    }

    fn initialization_fn(&self) -> Option<String> {
        Some(format!("{}.register", self.ffi_converter_instance()))
    }
}
