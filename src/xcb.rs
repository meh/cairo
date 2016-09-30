// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;
use glib::translate::*;
use ffi::{self, xcb};
use {Surface, Device};

pub fn surface(connection: &xcb::Connection,
               drawable: xcb::Drawable,
               visual: &xcb::Visualtype,
               width: u32,
               height: u32) -> Surface {
    Surface(unsafe { ffi::cairo_xcb_surface_create(
        connection.get_raw_conn(),
        drawable, visual.ptr,
        width as c_int, height as c_int) })
}

pub fn surface_for_bitmap(connection: &xcb::Connection,
                          screen: &xcb::Screen,
                          bitmap: xcb::Pixmap,
                          width: u32,
                          height: u32) -> Surface {
Surface(unsafe { ffi::cairo_xcb_surface_create_for_bitmap(
    connection.get_raw_conn(),
    screen.ptr, bitmap,
    width as c_int, height as c_int) })
}

pub fn surface_with_format(connection: &xcb::Connection,
                           screen: &xcb::Screen,
                           drawable: xcb::Drawable,
                           format: &xcb::render::Pictforminfo,
                           width: u32,
                           height: u32) -> Surface {
  Surface(unsafe { ffi::cairo_xcb_surface_create_with_xrender_format(
      connection.get_raw_conn(),
      screen.ptr, drawable, format.ptr,
      width as c_int, height as c_int) })
}

pub trait XcbSurfaceExt {
  fn set_size(&self, width: u32, height: u32);
  fn set_drawable(&self, drawable: xcb::Drawable, width: u32, height: u32);
}

impl XcbSurfaceExt for Surface {
  fn set_size(&self, width: u32, height: u32) {
    unsafe { ffi::cairo_xcb_surface_set_size(self.to_glib_none().0, width as c_int, height as c_int) }
  }

  fn set_drawable(&self, drawable: xcb::Drawable, width: u32, height: u32) {
    unsafe { ffi::cairo_xcb_surface_set_drawable(self.to_glib_none().0, drawable, width as c_int, height as c_int) }
  }
}

pub trait XcbDeviceExt {
  fn get_connection(&self) -> *mut xcb::ffi::xcb_connection_t;
}

impl XcbDeviceExt for Device {
  fn get_connection(&self) -> *mut xcb::ffi::xcb_connection_t {
    unsafe { ffi::cairo_xcb_device_get_connection(self.to_glib_none().0) }
  }
}
