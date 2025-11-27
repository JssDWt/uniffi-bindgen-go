/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use uniffi_bindgen::{backend::Literal, interface::Type, ComponentInterface};

use super::{filters::oracle, CodeType};

#[derive(Debug)]
pub struct EnumCodeType {
    name: String,
    module_path: String,
}

impl EnumCodeType {
    pub fn new(name: String, module_path: String) -> Self {
        Self { name, module_path }
    }
}

impl CodeType for EnumCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        let mut name = oracle().class_name(&self.name);

        // Check if this is an external type
        let type_ref = Type::Enum {
            module_path: self.module_path.clone(),
            name: self.name.clone(),
        };

        if ci.is_external(&type_ref) {
            // Get the namespace for the external type
            if let Ok(namespace) = ci.namespace_for_module_path(&self.module_path) {
                name = format!("{}.{}", namespace, name);
            }
        }

        if ci.is_name_used_as_error(&self.name) {
            format!("*{name}")
        } else {
            name
        }
    }

    fn canonical_name(&self) -> String {
        oracle().class_name(&self.name)
    }

    fn literal(&self, literal: &Literal, ci: &ComponentInterface) -> String {
        if let Literal::Enum(v, _) = literal {
            format!(
                "{}.{}",
                self.type_label(ci),
                super::GoCodeOracle.enum_variant_name(v)
            )
        } else {
            unreachable!();
        }
    }
}
