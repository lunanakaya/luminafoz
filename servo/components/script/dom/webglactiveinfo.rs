/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLActiveInfoBinding;
use dom::bindings::codegen::Bindings::WebGLActiveInfoBinding::WebGLActiveInfoMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{Temporary, JSRef};
use dom::bindings::utils::{Reflector,reflect_dom_object};
use util::str::DOMString;

#[dom_struct]
pub struct WebGLActiveInfo {
    reflector_: Reflector,
    size: i32,
    // NOTE: `ty` stands for `type`, which is a reserved keyword
    ty: u32,
    name: String,
}

impl WebGLActiveInfo {
    fn new_inherited(size: i32, ty: u32, name: String) -> WebGLActiveInfo {
        WebGLActiveInfo {
            reflector_: Reflector::new(),
            size: size,
            ty: ty,
            name: name,
        }
    }

    pub fn new(global: GlobalRef, size: i32, ty: u32, name: String) -> Temporary<WebGLActiveInfo> {
        reflect_dom_object(box WebGLActiveInfo::new_inherited(size, ty, name), global, WebGLActiveInfoBinding::Wrap)
    }
}

impl<'a> WebGLActiveInfoMethods for JSRef<'a, WebGLActiveInfo> {
    fn Size(self) -> i32 {
        self.size
    }

    fn Type(self) -> u32 {
        self.ty
    }

    fn Name(self) -> DOMString {
        self.name.clone()
    }
}
