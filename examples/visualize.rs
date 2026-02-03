use p_dispersion::{Point, solve_p_dispersion};
use porcelain::{color::Color, conf::WindowConfig, render::EventListener, start};

struct AppState {
    draw_index: Box<[usize]>,
    draw_location: Box<[porcelain::Point]>,
}

impl EventListener for AppState {
    fn update(&mut self, _texture_context: &porcelain::texture::TextureContext, _dt: f64) {
        // no-op
    }

    fn draw(&self, draw_context: &mut porcelain::render::DrawContext) {
        // Draw all circle once, then over draw with overlay on top for selected ones
        self.draw_location.iter().for_each(|location| {
            draw_context.draw_circle(location, 50., Color::from_rgba8(128, 128, 128, 255));
        });

        self.draw_index.iter().for_each(|index| {
            draw_context.draw_circle(&self.draw_location[*index], 40., Color::from_rgba8(116, 199, 236, 255));
        });
    }
}

fn main() {
    let window_config = WindowConfig {
        window_title: "Visualize P Dispersion".to_owned(),
        fullscreen: true,
        ..Default::default()
    };

    let points = [
        Point::new(0., 0.),
        Point::new(1., 0.),
        Point::new(2., 0.),
        Point::new(3., 0.),
        Point::new(4., 0.),
        Point::new(0., 1.),
        Point::new(1., 1.),
        Point::new(2., 1.),
        Point::new(3., 1.),
        Point::new(4., 1.),
        Point::new(0., 2.),
        Point::new(1., 2.),
        Point::new(2., 2.),
        Point::new(3., 2.),
        Point::new(4., 2.),
        Point::new(0., 3.),
        Point::new(1., 3.),
        Point::new(2., 3.),
        Point::new(3., 3.),
        Point::new(4., 3.),
    ];

    let draw_points = points.map(|data| -> porcelain::Point {
        porcelain::Point {
            x: data.x * 200. + 100.,
            y: data.y * 200. + 100.,
        }
    });

    let app_state = AppState {
        draw_index: solve_p_dispersion(&points, 6).expect("The problem is solved."),
        draw_location: Box::new(draw_points),
    };

    start(window_config, app_state);
}
