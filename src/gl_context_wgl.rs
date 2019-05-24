/*
 * Copyright 2015 The Servo Project Developers
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

use gl_rasterization_context;
use skia;

use euclid::Size2D;
use gleam::gl;
use std::ptr;
use std::rc::Rc;

pub struct PlatformDisplayData;

impl PlatformDisplayData {
    pub fn new() -> PlatformDisplayData {
        PlatformDisplayData
    }
}

pub struct GLPlatformContext {
    gl: Rc<gl::Gl>,
    pub context: Option<()>,

    pub framebuffer_id: gl::GLuint,
    texture_id: gl::GLuint,
    depth_stencil_renderbuffer_id: gl::GLuint,
}

impl Drop for GLPlatformContext {
    fn drop(&mut self) {
        self.make_current();
        gl_rasterization_context::destroy_framebuffer(self.gl(),
                                                      self.framebuffer_id,
                                                      self.texture_id,
                                                      self.depth_stencil_renderbuffer_id);
        self.destroy();
    }
}

impl GLPlatformContext {
    pub fn new(gl: Rc<gl::Gl>,
               _: PlatformDisplayData,
               size: Size2D<i32>)
               -> Option<GLPlatformContext> {
                   None
    }

    fn gl(&self) -> &gl::Gl {
        &*self.gl
    }

    pub fn drop_current_context(&self) {
        // TODO; should not be necessary
    }

    pub fn destroy(&self) {
        // TODO; need to extend glutin
    }

    pub fn make_current(&self) {
    }
}
