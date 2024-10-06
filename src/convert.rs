//! Concretizes [`relative::Layout`] into [`absolute::Layout`]

use crate::{
    absolute,
    comms::{self, Comms},
    geometry::Rect,
    relative::{self, Position},
};

impl relative::Layout {
    /// Resolve the layout according to the currently connected displays.
    pub fn to_absolute(&self, comms: &mut dyn Comms) -> comms::Result<absolute::Layout> {
        let mut placed = absolute::Layout::new();
        let current = comms.layout()?;
        let mut bb = Rect::default();

        for screen in &self.screens {
            // TODO: this manual merging logic is a bit strenous.
            // maybe this could be done shorter somehow?
            let screen_in_sway = current.outputs.get(&screen.port);

            let scale = screen
                .scale
                .or_else(|| screen_in_sway.map(|cfg| cfg.scale))
                .unwrap_or(1.0);

            let resolution = screen
                .resolution
                .map(|res| res.size())
                .or_else(|| screen_in_sway.map(|cfg| cfg.bounds.size() * scale));
            let Some(resolution) = resolution else {
                // user specified screen that isn't connected
                // hence should not affect layout
                continue;
            };

            // Which size the screen occupies in the *layout*, not physically.
            // See the manual page of sway-output for why the scale division is done.
            // In short: For positioning, the scale has to be taken into account.
            // So if screen A has scale 2 and has a resolution of 800x600,
            // and we wanted to place screen B right next to it,
            // we'd need to place it at 400x0 (since the scale is 2, and 800 / 2 = 400).
            // In our case, that just means dividing the size of the bounds by the scale,
            // then using it accordingly in the bounding box.
            let layout_size = resolution.rotate(screen.transform.rotation) / scale;

            // note: order of x/y placement does not actually matter
            // they don't have any influence on each other
            let bounds = match screen.pos {
                // place left/right of bbox, then decide exact vertical placement
                Position::Hori { edge, spec } => Rect {
                    x: bb.x.place_outside(layout_size.width, edge.into()),
                    y: bb.y.place_inside(layout_size.height, spec.map(Into::into)),
                },
                // place top/bottom of bbox, then decide exact horizontal placement
                Position::Vert { edge, spec } => Rect {
                    x: bb.x.place_inside(layout_size.width, spec.map(Into::into)),
                    y: bb.y.place_outside(layout_size.height, edge.into()),
                },
            };

            // now that we've got the screen bounds, make sure it's actually noticed
            // by the bounding box
            // so future screens can be placed accordingly
            bb.stretch_to_rect(bounds);

            // that'd be it! let's actually place the output screen
            // we just calculated the bounds of
            placed.add(absolute::Output {
                port: screen.port,
                cfg: absolute::OutputConfig {
                    bounds,
                    scale,
                    resolution: Some(resolution),
                    transform: screen.transform,
                    active: true,
                },
            });
        }

        placed.reset_to_origin();

        Ok(placed)
    }
}
