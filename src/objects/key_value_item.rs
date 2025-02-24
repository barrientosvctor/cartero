// Copyright 2024 the Cartero authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use glib::Object;

mod imp {
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::glib::prelude::*;
    use gtk::glib::subclass::prelude::*;

    #[derive(Default, Debug, Properties)]
    #[properties(wrapper_type = super::KeyValueItem)]
    pub struct KeyValueItem {
        #[property(get, set)]
        active: RefCell<bool>,
        #[property(get, set)]
        secret: RefCell<bool>,

        #[property(get, set)]
        header_name: RefCell<String>,
        #[property(get, set)]
        header_value: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KeyValueItem {
        const NAME: &'static str = "CarteroKeyValueItem";
        type Type = super::KeyValueItem;
    }

    #[glib::derived_properties]
    impl ObjectImpl for KeyValueItem {}
}

glib::wrapper! {
    pub struct KeyValueItem(ObjectSubclass<imp::KeyValueItem>);
}

impl KeyValueItem {
    // For a header to be actually usable, it must be checked, and also it must have a header name
    // properly set. We could argue that having an empty value is also dumb, but the spec
    // technically allows this.
    pub fn is_usable(&self) -> bool {
        self.active() && !self.header_name().is_empty()
    }
}

impl Default for KeyValueItem {
    fn default() -> Self {
        Object::builder().build()
    }
}

#[cfg(test)]
mod tests {
    use super::KeyValueItem;

    #[test]
    pub fn test_header_properties() {
        let header = KeyValueItem::default();
        assert_eq!(header.header_name(), "");
        assert_eq!(header.header_value(), "");
        assert!(!header.active());

        header.set_header_name("Content-Type");
        header.set_header_value("text/plain");
        header.set_active(true);
        assert_eq!(header.header_name(), "Content-Type");
        assert_eq!(header.header_value(), "text/plain");
        assert!(header.active());

        header.set_header_name("Accept");
        assert_eq!(header.header_name(), "Accept");
        assert_eq!(header.header_value(), "text/plain");
        assert!(header.active());
    }
}
