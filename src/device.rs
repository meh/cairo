// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use ffi::enums::{
    Status,
    DeviceType,
};

#[derive(Debug)]
pub struct Device(pub *mut ffi::cairo_device_t);

impl Device {
  pub fn status(&self) -> Status {
    unsafe { ffi::cairo_device_status(self.0) }
  }
}

impl<'a> ToGlibPtr<'a, *mut ffi::cairo_device_t> for Device {
    type Storage = &'a Device;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_device_t, Self> {
        Stash(self.0, self)
    }
}

impl FromGlibPtr<*mut ffi::cairo_device_t> for Device {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        ffi::cairo_device_reference(ptr);
        Device(ptr)
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr)
    }
}

impl AsRef<Device> for Device {
    fn as_ref(&self) -> &Device {
        self
    }
}

impl Clone for Device {
    fn clone(&self) -> Device {
        unsafe { from_glib_none(self.to_glib_none().0) }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { ffi::cairo_device_destroy(self.0); }
    }
}

pub trait DeviceExt {
  fn flush(&self);
  fn finish(&self);
  fn get_type(&self) -> DeviceType;
}

impl<O: AsRef<Device>> DeviceExt for O {
  fn flush(&self) {
    unsafe { ffi::cairo_device_flush(self.as_ref().0) }
  }

  fn finish(&self) {
    unsafe { ffi::cairo_device_finish(self.as_ref().0) }
  }

  fn get_type(&self) -> DeviceType {
    unsafe { ffi::cairo_device_get_type(self.as_ref().0) }
  }
}
