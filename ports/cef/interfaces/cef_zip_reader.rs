// Copyright (c) 2015 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::mem;
use std::ptr;

//
// Structure that supports the reading of zip archives via the zlib unzip API.
// The functions of this structure should only be called on the thread that
// creates the object.
//
#[repr(C)]
pub struct _cef_zip_reader_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Moves the cursor to the first file in the archive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub move_to_first_file: Option<extern "C" fn(
      this: *mut cef_zip_reader_t) -> libc::c_int>,

  //
  // Moves the cursor to the next file in the archive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub move_to_next_file: Option<extern "C" fn(
      this: *mut cef_zip_reader_t) -> libc::c_int>,

  //
  // Moves the cursor to the specified file in the archive. If |caseSensitive|
  // is true (1) then the search will be case sensitive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub move_to_file: Option<extern "C" fn(this: *mut cef_zip_reader_t,
      fileName: *const types::cef_string_t,
      caseSensitive: libc::c_int) -> libc::c_int>,

  //
  // Closes the archive. This should be called directly to ensure that cleanup
  // occurs on the correct thread.
  //
  pub close: Option<extern "C" fn(this: *mut cef_zip_reader_t) -> libc::c_int>,


  // The below functions act on the file at the current cursor position.

  //
  // Returns the name of the file.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_file_name: Option<extern "C" fn(
      this: *mut cef_zip_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the uncompressed size of the file.
  //
  pub get_file_size: Option<extern "C" fn(this: *mut cef_zip_reader_t) -> i64>,

  //
  // Returns the last modified timestamp for the file.
  //
  pub get_file_last_modified: Option<extern "C" fn(
      this: *mut cef_zip_reader_t) -> types::cef_time_t>,

  //
  // Opens the file for reading of uncompressed data. A read password may
  // optionally be specified.
  //
  pub open_file: Option<extern "C" fn(this: *mut cef_zip_reader_t,
      password: *const types::cef_string_t) -> libc::c_int>,

  //
  // Closes the file.
  //
  pub close_file: Option<extern "C" fn(
      this: *mut cef_zip_reader_t) -> libc::c_int>,

  //
  // Read uncompressed file contents into the specified buffer. Returns < 0 if
  // an error occurred, 0 if at the end of file, or the number of bytes read.
  //
  pub read_file: Option<extern "C" fn(this: *mut cef_zip_reader_t,
      buffer: *mut (), bufferSize: libc::size_t) -> libc::c_int>,

  //
  // Returns the current offset in the uncompressed file contents.
  //
  pub tell: Option<extern "C" fn(this: *mut cef_zip_reader_t) -> i64>,

  //
  // Returns true (1) if at end of the file contents.
  //
  pub eof: Option<extern "C" fn(this: *mut cef_zip_reader_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_zip_reader_t = _cef_zip_reader_t;


//
// Structure that supports the reading of zip archives via the zlib unzip API.
// The functions of this structure should only be called on the thread that
// creates the object.
//
pub struct CefZipReader {
  c_object: *mut cef_zip_reader_t,
}

impl Clone for CefZipReader {
  fn clone(&self) -> CefZipReader{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefZipReader {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefZipReader {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefZipReader {
  pub unsafe fn from_c_object(c_object: *mut cef_zip_reader_t) -> CefZipReader {
    CefZipReader {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_zip_reader_t) -> CefZipReader {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefZipReader {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_zip_reader_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_zip_reader_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Moves the cursor to the first file in the archive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub fn move_to_first_file(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_first_file.unwrap())(
          self.c_object))
    }
  }

  //
  // Moves the cursor to the next file in the archive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub fn move_to_next_file(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_next_file.unwrap())(
          self.c_object))
    }
  }

  //
  // Moves the cursor to the specified file in the archive. If |caseSensitive|
  // is true (1) then the search will be case sensitive. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub fn move_to_file(&self, fileName: &[u16],
      caseSensitive: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_file.unwrap())(
          self.c_object,
          CefWrap::to_c(fileName),
          CefWrap::to_c(caseSensitive)))
    }
  }

  //
  // Closes the archive. This should be called directly to ensure that cleanup
  // occurs on the correct thread.
  //
  pub fn close(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).close.unwrap())(
          self.c_object))
    }
  }


  // The below functions act on the file at the current cursor position.

  //
  // Returns the name of the file.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_file_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the uncompressed size of the file.
  //
  pub fn get_file_size(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_size.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the last modified timestamp for the file.
  //
  pub fn get_file_last_modified(&self) -> types::cef_time_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_last_modified.unwrap())(
          self.c_object))
    }
  }

  //
  // Opens the file for reading of uncompressed data. A read password may
  // optionally be specified.
  //
  pub fn open_file(&self, password: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).open_file.unwrap())(
          self.c_object,
          CefWrap::to_c(password)))
    }
  }

  //
  // Closes the file.
  //
  pub fn close_file(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).close_file.unwrap())(
          self.c_object))
    }
  }

  //
  // Read uncompressed file contents into the specified buffer. Returns < 0 if
  // an error occurred, 0 if at the end of file, or the number of bytes read.
  //
  pub fn read_file(&self, buffer: &mut (),
      bufferSize: libc::size_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).read_file.unwrap())(
          self.c_object,
          CefWrap::to_c(buffer),
          CefWrap::to_c(bufferSize)))
    }
  }

  //
  // Returns the current offset in the uncompressed file contents.
  //
  pub fn tell(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).tell.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if at end of the file contents.
  //
  pub fn eof(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).eof.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_zip_reader_t object. The returned object's functions can
  // only be called from the thread that created the object.
  //
  pub fn create(
      stream: interfaces::CefStreamReader) -> interfaces::CefZipReader {
    unsafe {
      CefWrap::to_rust(
        ::zip_reader::cef_zip_reader_create(
          CefWrap::to_c(stream)))
    }
  }
} 

impl CefWrap<*mut cef_zip_reader_t> for CefZipReader {
  fn to_c(rust_object: CefZipReader) -> *mut cef_zip_reader_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_zip_reader_t) -> CefZipReader {
    CefZipReader::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_zip_reader_t> for Option<CefZipReader> {
  fn to_c(rust_object: Option<CefZipReader>) -> *mut cef_zip_reader_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_zip_reader_t) -> Option<CefZipReader> {
    if c_object.is_null() {
      None
    } else {
      Some(CefZipReader::from_c_object_addref(c_object))
    }
  }
}

