use std::{fmt::{self, Display, Formatter}, f32::consts::PI};

pub struct Path {
    inner: String,
}

impl Path {
    pub fn new() -> Path { Path { inner: String::new() } }

    pub fn move_to(mut self, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("M {} {}", x, y)).unwrap();
        self
    }

    pub fn move_by(mut self, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("m {} {}", dx, dy)).unwrap();
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("L {} {}", x, y)).unwrap();
        self
    }

    pub fn line_by(mut self, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("l {} {}", dx, dy)).unwrap();
        self
    }

    pub fn horizontal_line_to(mut self, x: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("H {}", x)).unwrap();
        self
    }

    pub fn horizontal_line_by(mut self, dx: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("h {}", dx)).unwrap();
        self
    }

    pub fn vertical_line_to(mut self, y: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("V {}", y)).unwrap();
        self
    }

    pub fn vertical_line_by(mut self, y: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("v {}", y)).unwrap();
        self
    }

    pub fn close(mut self) -> Path {
        fmt::write(&mut self.inner, format_args!("Z")).unwrap();
        self
    }

    pub fn cubic_bezier_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("C {} {}, {} {}, {} {}", x1, y1, x2, y2, x, y)
        ).unwrap();
        self
    }

    pub fn cubic_bezier_by(mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("c {} {}, {} {}, {} {}", dx1, dy1, dx2, dy2, dx, dy)
        ).unwrap();
        self
    }

    pub fn smooth_cubic_bezier_to(mut self, x2: f32, y2: f32, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("S {} {}, {} {}", x2, y2, x, y)
        ).unwrap();
        self
    }

    pub fn smooth_cubic_bezier_by(mut self, dx2: f32, dy2: f32, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("s {} {}, {} {}", dx2, dy2, dx, dy)
        ).unwrap();
        self
    }

    pub fn quadratic_bezier_to(mut self, x1: f32, y1: f32, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("Q {} {}, {} {}", x1, y1, x, y)
        ).unwrap();
        self
    }

    pub fn quadratic_bezier_by(mut self, dx1: f32, dy1: f32, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, 
            format_args!("q {} {}, {} {}", dx1, dy1, dx, dy)
        ).unwrap();
        self
    }

    pub fn smooth_quadratic_cubic_bezier_to(mut self, x: f32, y: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("T {} {}", x, y)).unwrap();
        self
    }

    pub fn smooth_quadratic_cubic_bezier_by(mut self, dx: f32, dy: f32) -> Path {
        fmt::write(&mut self.inner, format_args!("t {} {}", dx, dy)).unwrap();
        self
    }

    pub fn elliptical_arc_to(mut self, rx: f32, ry: f32, xrot: f32, large_arc: bool, sweep: bool, x: f32, y: f32) -> Path {
        let large_arc = if large_arc { 1 } else { 0 };
        let sweep = if sweep { 1 } else { 0 };
        fmt::write(&mut self.inner, 
            format_args!("A {} {} {} {} {} {} {}", rx, ry, xrot, large_arc, sweep, x, y)
        ).unwrap();
        self
    }

    pub fn elliptical_arc_by(mut self, rx: f32, ry: f32, xrot: f32, large_arc: bool, sweep: bool, dx: f32, dy: f32) -> Path {
        let large_arc = if large_arc { 1 } else { 0 };
        let sweep = if sweep { 1 } else { 0 };
        fmt::write(&mut self.inner, 
            format_args!("A {} {} {} {} {} {} {}", rx, ry, xrot, large_arc, sweep, dx, dy)
        ).unwrap();
        self
    }

    pub fn partial_circle(center_x: f32, center_y: f32, radius: f32, start_angle: f32, arc_angle: f32) -> Path {
        let mut path = Path::new()
            .move_to(center_x + radius * start_angle.cos(), center_y - radius * start_angle.sin());

        let mid_angle = start_angle + arc_angle.signum() * arc_angle.abs().min(PI);
        path = path.elliptical_arc_to(
            radius, radius, 0f32, false, arc_angle < 0.0,
            center_x + radius * mid_angle.cos(), center_y - radius * mid_angle.sin()
        );
        
        if arc_angle.abs() > PI {
            let end_angle = start_angle + arc_angle;
            path = path.elliptical_arc_to(
                radius, radius, 0f32, false, arc_angle < 0.0, 
                center_x + radius * end_angle.cos(), center_y - radius * end_angle.sin()
            );
        }

        path
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}