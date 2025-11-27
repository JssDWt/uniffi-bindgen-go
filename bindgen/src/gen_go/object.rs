/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use uniffi_bindgen::{backend::Literal, interface::Type, ComponentInterface};
use uniffi_meta::ObjectImpl;

use super::{filters::oracle, CodeType};

#[derive(Debug)]
pub struct ObjectCodeType {
    id: String,
    imp: ObjectImpl,
    module_path: String,
}

impl ObjectCodeType {
    pub fn new(id: String, imp: ObjectImpl, module_path: String) -> Self {
        Self { id, imp, module_path }
    }
}

impl CodeType for ObjectCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        let mut class_name = oracle().class_name(&self.id);
        
        // Check if this is an external type
        let type_ref = Type::Object {
            module_path: self.module_path.clone(),
            name: self.id.clone(),
            imp: self.imp,
        };
        
        if ci.is_external(&type_ref) {
            // Get the namespace for the external type
            if let Ok(namespace) = ci.namespace_for_module_path(&self.module_path) {
                class_name = format!("{}.{}", namespace, class_name);
            }
        }
        
        if self.imp.has_callback_interface() {
            // When object has callback interface, it is represented
            // as interface, that is already a fat pointer
            class_name
        } else {
            format!("*{}", class_name)
        }
    }

    fn canonical_name(&self) -> String {
        oracle().class_name(&self.id)
    }

    fn literal(&self, _literal: &Literal, _ci: &ComponentInterface) -> String {
        unreachable!();
    }

    fn initialization_fn(&self) -> Option<String> {
        self.imp
            .has_callback_interface()
            .then(|| format!("{}.register", self.ffi_converter_instance()))
    }
}
