//! Custom GTK widget for rendering non-rectangular and rich-content keys.
//!
//! Supports L-shaped keys (ISO Enter), tall keys (numpad +/Enter), and
//! custom widget types (color picker, image, piano) via Cairo drawing.

use cairo::Context;
use gtk4::Snapshot;
use gtk4::glib;
use gtk4::graphene;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use osk_core::CustomWidget;
use osk_core::KeyShape;
use std::cell::RefCell;

mod imp {
    use super::*;

    /// Internal state for the custom key widget.
    #[derive(Default)]
    pub struct CustomKeyWidget {
        /// Label text displayed on the key
        pub label: RefCell<String>,
        /// Geometric shape of the key
        pub shape: RefCell<KeyShape>,
        /// Custom widget type for rich rendering
        pub custom: RefCell<CustomWidget>,
        /// Optional CSS class for styling
        pub css_class: RefCell<Option<String>>,
        /// Whether the key is currently pressed
        pub pressed: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomKeyWidget {
        const NAME: &'static str = "OskCustomKeyWidget";
        type Type = super::CustomKeyWidget;
        type ParentType = gtk4::Widget;
    }

    impl ObjectImpl for CustomKeyWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.add_css_class("key");
        }
    }

    impl WidgetImpl for CustomKeyWidget {
        fn measure(&self, orientation: gtk4::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            // Minimum size: 40x40 per grid unit, natural: 60x60
            let shape = self.shape.borrow();
            let min = 40;
            let natural = match orientation {
                gtk4::Orientation::Horizontal => (shape.width_u() * 60.0) as i32,
                gtk4::Orientation::Vertical => (shape.height_rows() as f32 * 60.0) as i32,
                _ => 60,
            };
            (min, natural.max(min), -1, -1)
        }

        fn snapshot(&self, snapshot: &Snapshot) {
            let obj = self.obj();
            let width = WidgetExt::width(obj.as_ref()) as f64;
            let height = WidgetExt::height(obj.as_ref()) as f64;
            if width <= 0.0 || height <= 0.0 {
                return;
            }

            let label = self.label.borrow();
            let shape = self.shape.borrow();
            let pressed = *self.pressed.borrow();
            let custom = self.custom.borrow();

            let bounds = graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&bounds);
            super::CustomKeyWidget::draw_key(&cr, width, height, &label, &shape, &custom, pressed);
        }

        fn contains(&self, x: f64, y: f64) -> bool {
            let obj = self.obj();
            let width = WidgetExt::width(obj.as_ref()) as f64;
            let height = WidgetExt::height(obj.as_ref()) as f64;
            let shape = self.shape.borrow();
            super::CustomKeyWidget::hit_test(&shape, x, y, width, height)
        }
    }
}

glib::wrapper! {
    /// Custom GTK widget for rendering non-rectangular and rich-content keys.
    ///
    /// Renders L-shaped keys (ISO Enter), tall keys (numpad +/Enter), and
    /// custom widget types via Cairo drawing. Hit-testing is shape-aware:
    /// clicks in the L-shape notch are ignored.
    pub struct CustomKeyWidget(ObjectSubclass<imp::CustomKeyWidget>)
        @extends gtk4::Widget,
        @implements gtk4::Buildable, gtk4::ConstraintTarget;
}

impl CustomKeyWidget {
    /// Create a new custom key widget with the given label, shape, and custom widget type.
    pub fn new(label: &str, shape: KeyShape, custom: CustomWidget, css_class: Option<&str>) -> Self {
        let widget: Self = glib::Object::builder().build();
        *widget.imp().label.borrow_mut() = label.to_string();
        *widget.imp().shape.borrow_mut() = shape;
        *widget.imp().custom.borrow_mut() = custom;
        if let Some(class) = css_class {
            widget.add_css_class(class);
            *widget.imp().css_class.borrow_mut() = Some(class.to_string());
        }
        widget
    }

    /// Set the pressed state for visual feedback.
    pub fn set_pressed(&self, pressed: bool) {
        *self.imp().pressed.borrow_mut() = pressed;
        if pressed {
            self.add_css_class("key-pressed");
        } else {
            self.remove_css_class("key-pressed");
        }
        self.queue_draw();
    }

    /// Draw the key using Cairo.
    fn draw_key(cr: &Context, width: f64, height: f64, label: &str, shape: &KeyShape, custom: &CustomWidget, pressed: bool) {
        match shape {
            KeyShape::Rect { .. } | KeyShape::Tall { .. } => {
                Self::draw_rounded_rect(cr, 0.0, 0.0, width, height, 6.0);
            }
            KeyShape::LShape { top_width_u, bottom_width_u } => {
                Self::draw_lshape(cr, width, height, *top_width_u, *bottom_width_u);
            }
        }

        if pressed {
            cr.set_source_rgba(0.29, 0.62, 1.0, 1.0);
        } else {
            cr.set_source_rgba(0.23, 0.23, 0.23, 1.0);
        }
        cr.fill().expect("Failed to fill key background");

        cr.set_source_rgba(0.35, 0.35, 0.35, 1.0);
        cr.set_line_width(1.0);
        match shape {
            KeyShape::Rect { .. } | KeyShape::Tall { .. } => {
                Self::draw_rounded_rect(cr, 0.0, 0.0, width, height, 6.0);
            }
            KeyShape::LShape { top_width_u, bottom_width_u } => {
                Self::draw_lshape(cr, width, height, *top_width_u, *bottom_width_u);
            }
        }
        cr.stroke().expect("Failed to stroke key border");

        match custom {
            CustomWidget::None => {
                if !label.is_empty() {
                    Self::draw_label(cr, width, height, label);
                }
            }
            CustomWidget::Custom(id) => {
                Self::draw_custom(cr, width, height, id);
            }
        }
    }

    /// Draw a rounded rectangle path.
    fn draw_rounded_rect(cr: &Context, x: f64, y: f64, width: f64, height: f64, radius: f64) {
        let r = radius.min(width / 2.0).min(height / 2.0);
        cr.new_path();
        cr.arc(x + width - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
        cr.arc(x + width - r, y + height - r, r, 0.0, std::f64::consts::FRAC_PI_2);
        cr.arc(x + r, y + height - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
        cr.arc(x + r, y + r, r, std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2);
        cr.close_path();
    }

    /// Draw an L-shaped path for ISO Enter.
    ///
    /// The L-shape consists of a top part (full width) and a bottom part
    /// (right-aligned, narrower). The notch (bottom-left) is transparent.
    fn draw_lshape(cr: &Context, width: f64, height: f64, top_width_u: f32, bottom_width_u: f32) {
        let total_u = top_width_u.max(bottom_width_u) as f64;
        let unit_w = width / total_u;
        let top_w = top_width_u as f64 * unit_w;
        let bottom_w = bottom_width_u as f64 * unit_w;
        let half_h = height / 2.0;
        let bottom_x = width - bottom_w;

        cr.new_path();
        cr.move_to(0.0, 0.0);
        cr.line_to(top_w, 0.0);
        cr.line_to(top_w, half_h);
        cr.line_to(width, half_h);
        cr.line_to(width, height);
        cr.line_to(bottom_x, height);
        cr.line_to(bottom_x, half_h);
        cr.line_to(0.0, half_h);
        cr.close_path();
    }

    /// Draw a text label centered in the widget.
    fn draw_label(cr: &Context, width: f64, height: f64, label: &str) {
        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        let font_size = (height / 4.0).clamp(10.0, 16.0);
        cr.set_font_size(font_size);

        let extents = cr.text_extents(label).expect("Failed to get text extents");
        let tx = (width - extents.width()) / 2.0 - extents.x_bearing();
        let ty = (height - extents.height()) / 2.0 - extents.y_bearing();
        cr.move_to(tx, ty);
        cr.show_text(label).expect("Failed to draw text");
    }

    /// Draw custom widget content based on a loose-coupling identifier.
    fn draw_custom(cr: &Context, width: f64, height: f64, id: &str) {
        match id {
            "color_picker" => {
                let size = width.min(height) * 0.6;
                let x = (width - size) / 2.0;
                let y = (height - size) / 2.0;
                cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
                Self::draw_rounded_rect(cr, x, y, size, size, 4.0);
                cr.fill().expect("Failed to fill color picker");
            }
            "piano" => {
                let octaves = 2u32;
                let white_count = octaves as f64 * 7.0;
                let white_w = width / white_count;
                let black_w = white_w * 0.6;
                let black_h = height * 0.6;

                for i in 0..octaves {
                    let x = i as f64 * 7.0 * white_w;
                    for j in 0..7 {
                        let wx = x + j as f64 * white_w;
                        cr.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                        Self::draw_rounded_rect(cr, wx + 1.0, 0.0, white_w - 2.0, height, 3.0);
                        cr.fill().expect("Failed to fill white key");
                    }
                }

                let black_positions = [0, 1, 3, 4, 5];
                for i in 0..octaves {
                    let base_x = i as f64 * 7.0 * white_w;
                    for &bp in &black_positions {
                        let bx = base_x + (bp as f64 + 1.0) * white_w - black_w / 2.0;
                        cr.set_source_rgba(0.15, 0.15, 0.15, 1.0);
                        Self::draw_rounded_rect(cr, bx, 0.0, black_w, black_h, 2.0);
                        cr.fill().expect("Failed to fill black key");
                    }
                }
            }
            _ => {
                cr.set_source_rgba(0.5, 0.5, 0.5, 1.0);
                cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                cr.set_font_size(10.0);
                let display = if id.len() > 12 { &id[..12] } else { id };
                let extents = cr.text_extents(display).expect("Failed to get text extents");
                let tx = (width - extents.width()) / 2.0 - extents.x_bearing();
                let ty = (height - extents.height()) / 2.0 - extents.y_bearing();
                cr.move_to(tx, ty);
                cr.show_text(display).expect("Failed to draw custom widget text");
            }
        }
    }

    /// Hit-test a point against the key shape.
    pub fn hit_test(shape: &KeyShape, x: f64, y: f64, width: f64, height: f64) -> bool {
        if x < 0.0 || y < 0.0 || x > width || y > height {
            return false;
        }
        match shape {
            KeyShape::Rect { .. } | KeyShape::Tall { .. } => true,
            KeyShape::LShape { top_width_u, bottom_width_u } => {
                let total_u = top_width_u.max(*bottom_width_u) as f64;
                let unit_w = width / total_u;
                let top_w = *top_width_u as f64 * unit_w;
                let half_h = height / 2.0;
                let bottom_w = *bottom_width_u as f64 * unit_w;
                let bottom_x = width - bottom_w;

                if y <= half_h { x <= top_w } else { x >= bottom_x && x <= width }
            }
        }
    }
}
