// http://gtk-rs.org/docs/requirements.html

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate cairo;
extern crate ___________; // TODO (a)
use gdk::ContextExt;
use gdk_pixbuf::{Colorspace::Rgb, Pixbuf, PixbufExt};
use gtk::prelude::*;
use gtk::{Window, WindowType};
use ___________::_______; // TODO (a)
use std::cell::RefCell;
use std::rc::Rc;

/// Computes the collatz function over the complex plane
pub fn collatz(c: _______<f64>) -> _______<f64>{ // TODO (a)
    let pi = ______; // TODO (a)
    _______::new(0.25, 0.0) * // TODO (a)
        (_______::new(1.0, 0.0) + _______::new(4.0, 0.0) * c - // TODO (a)
         (_______::new(1.0, 0.0) + _______::new(2.0, 0.0) * c) * (pi*c).cos()) // TODO (a)
}

/// Transforms the iteratively gained value from the interval [0.0, 1.0]
/// to an f64 number representing an RGB value
pub fn hue_to_rgb(mut h: f64) -> f64 {
    if h > 1.0 {
        h = 1.0;
    }
    255.0 * match h {
        h if h < 0.16 => 6.0 * h,
        h if h < 0.5 => 1.0,
        h if h < 0.67 => (0.67 - h) * 6.0,
        _ => 1.0
    }
}

/// Draws a fractal into a given Viewport as a side effect
pub fn fractal(vp: &mut Viewport) {
    let (max_iter, max_norm) = (2048, 512.0);
    let origin = vp.origin();
    let pixeldelta = vp.delta();
    let (width, height) = vp.pixelcount();
    for y in 0..height {
        for x in 0..width {
            let cx = x as f64 * pixeldelta.0 + origin.0;
            let cy = y as f64 * pixeldelta.1 + origin.1;

            let mut z = _______::new(cx, cy); // TODO (a)
            let mut i = 0;

            let mut delta = 1.0f64;
            while i < max_iter && z.norm() < max_norm && delta > 1e-6 {
                // TODO
                // TODO (b)
                // TODO
                i += 1;
            }
            let lum = (i as f64).ln()/(max_iter as f64).ln();
            let (r, g, b) = (hue_to_rgb(lum-0.3) as u8,
                             hue_to_rgb(lum-0.3) as u8,
                             hue_to_rgb(lum) as u8);
            vp.setpixel(x, y, 255-r, 255-g, 255-b);
        }
    }
}

type SharedView = Rc<RefCell<Viewport>>;

/// * Represents the rectangular section of the portion of the complex plane
///   currently in view.
/// * Maintains the dynamic mapping from (x,y) pixel coordinates to complex
///   numbers.
pub struct Viewport {
    x0: f64, y0: f64, x1: f64, y1: f64,
    pix: gdk_pixbuf::Pixbuf,
    stale: bool
}
impl Viewport {
    /// Create a new Viewport with a given width and height,
    /// covering the complex plane from -2.0 to +2.0
    pub fn new(width: i32, height: i32) -> Viewport {
        let ratio = height as f64/width as f64;
        Viewport {
            x0: -2.0, y0:  2.0*ratio,
            x1:  2.0, y1: -2.0*ratio,
            pix: Pixbuf::new(Rgb, false, 8, width, height),
            stale: true
        }
    }
    /// Left upper corner of the viewport
    pub fn origin(&self) -> (f64, f64) { (self.x0, self.y0) }
    /// Conversion factor from pixel to complex number
    pub fn delta(&self) -> (f64, f64) {
        ((self.x1-self.x0)/self.pix.get_width() as f64,
         (self.y1-self.y0)/self.pix.get_height() as f64)
    }
    /// Number of pixels in each direction
    pub fn pixelcount(&self) -> (i32, i32) {
        (self.pix.get_width(), self.pix.get_height())
    }
    /// Resizes the image to a given width and height
    pub fn resize(&mut self, width: i32, height: i32) {
        self.pix = Pixbuf::new(Rgb, false, 8, width, height);
        // TODO: do something with the coordinates (f)
    }
    /// Set the RGB values of a specific pixel
    pub fn setpixel(&self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        &self.pix.put_pixel(x, y, r, g, b, 0);
    }
    /// Print the complex numbers (as tuples) representing upper left corner
    /// and bottom right corner.
    pub fn viewport(&self) {
        println!("view: ({}, {}) -- ({}, {})",
                 self.x0, self.y0, self.x1, self.y1);
    }
}
/// Adjusts the viewport according to last mouse event
/// ((left click is zoom in, right click is zoom out).
/// Sets the viewport to ''stale'', which leads to the
/// fractal being re-calculated with the next draw.
fn mouse(widget: &gtk::DrawingArea, but: ___, x: f64, y: f64, v: SharedView) { // TODO (c)
    // widget dimensions
    let (w, h) = (widget.get_allocated_width(), widget.get_allocated_height());
    // mouse coordinates between 0..1 for left/top .. right/bottom
    let (rel_x, rel_y) = (x/w as f64, y/h as f64);
    // we need to change the viewport
    let mut vmut = v.borrow_mut();
    // old width and height in the viewport
    let (w_old, h_old) = (vmut.x1-vmut.x0, vmut.y1-vmut.y0);
    // mouse coordinates w.r.t. the viewport == new center
    let (cx, cy) = (vmut.x0+w_old*rel_x, vmut.y0+h_old*rel_y);
    // mouse button handling: zoom in/zoom out
    let (w_new, h_new) = match but {
        _?_ => (w_old/4.0, h_old/4.0), // TODO (c)
        _?_ => (w_old,     h_old),     // TODO (c)
        _ => return
    };
    vmut.x0 = cx - w_new; vmut.x1 = cx + w_new;
    vmut.y0 = cy - h_new; vmut.y1 = cy + h_new;
    vmut.viewport();
    vmut.stale = true;
    widget.queue_draw();
}

/// Fits the PixBuf to the Window Size. Handles redrawing when necessary
fn draw(widget: &gtk::DrawingArea, ctxt: &cairo::Context, v: SharedView) {
    let (w, h) = (widget.get_allocated_width(), widget.get_allocated_height());
    let mut vmut = v.borrow_mut();
    if (w, h) != vmut.pixelcount() {
        vmut.resize(w, h);
    }

    if vmut.stale {
        // TODO (d)
        // TODO (d)
    }
    ctxt.set_source_pixbuf(___); // TODO (d)
    ctxt.paint();
}

fn main() {
    gtk::init().expect("GTK initialization failed");
    let window = Window::new(WindowType::Toplevel);
    let (size_x, size_y)=(640, 480);
    window.set_title("Collatz fractal");
    window.set_default_size(size_x, size_y);
    window.connect_delete_event(|_, _| {
        _____; // TODO (e)
        Inhibit(false)});

    let v = Rc::new(RefCell::new(Viewport::new(size_x, size_y)));
    let w = v.clone();

    let canvas = gtk::DrawingArea::new();
    canvas.set_events(gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32);
    canvas.connect_button_press_event(move |w, m| {
        if let Some((mx, my)) = m.get_coords() {
            mouse(w, m.get_button(), mx, my, v.clone()); }
        Inhibit(false)
    });
    canvas.connect_draw(move |area, ctxt| {
        draw(&area, ctxt, w.clone()); Inhibit(false) });

    window.add(&canvas);
    window.show_all();
    gtk::main();
}
