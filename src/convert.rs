//! Concretizes [`relative::Layout`] into [`absolute::Layout`]

use crate::{
    absolute,
    comms::{self, Comms},
    geometry::{Corner, Hori, Interval, Rect, Vert},
    relative::{self, Position},
};

impl relative::Layout {
    /// Resolve the layout according to the currently connected displays.
    pub fn to_absolute(&self, comms: &mut dyn Comms) -> comms::Result<absolute::Layout> {
        let mut placed = absolute::Layout::new();
        let currently_active = comms.layout()?;
        let mut bb = Rect {
            x: Interval::new(0, 0),
            y: Interval::new(0, 0),
        };

        for screen in &self.screens {
            let resolution = screen.resolution.map(|res| res.size()).or_else(|| {
                currently_active
                    .outputs
                    .get(&screen.port)
                    .map(|cfg| cfg.bounds.size())
            });
            let Some(resolution) = resolution else {
                // user specified screen that isn't connected
                // hence should not affect layout
                continue;
            };

            // note: order of x/y placement does not actually matter
            // they don't have any influence on each other
            let mut bounds = match screen.pos {
                // place left/right of bbox, then decide exact vertical placement
                Position::Hori { edge, spec } => Rect {
                    x: bb.x.place_outside(resolution.width, edge.into()),
                    y: bb.y.place_inside(resolution.height, spec.map(Into::into)),
                },
                // place top/bottom of bbox, then decide exact horizontal placement
                Position::Vert { edge, spec } => Rect {
                    x: bb.x.place_inside(resolution.width, spec.map(Into::into)),
                    y: bb.y.place_outside(resolution.height, edge.into()),
                },
            };

            // See the manual page of sway-output.
            // For positioning, the scale has to be taken into account.
            // So if screen A has scale 2 and has a resolution of 800x600,
            // and we wanted to place screen B right next to it,
            // we'd need to place it at 400x0 (since the scale is 2, and 800 / 2 = 400).
            // In our case, that just means dividing the size of the bounds by the scale,
            // then using it accordingly in the bounding box.
            bounds.divide_at(
                Corner {
                    hori: Hori::Left,
                    vert: Vert::Top,
                },
                // TODO: maybe get the scale from Sway as fallback first?
                screen.scale.unwrap_or(1.0),
            );

            // now that we've got the screen bounds, make sure it's actually noticed
            // by the bounding box
            // so future screens can be placed accordingly
            // TODO: apply scale to the bounds as seen by the bb?
            bb.stretch_to_rect(bounds);

            // that'd be it! let's actually place the output screen
            // we just calculated its bounds of
            let scale = screen.scale.unwrap_or({
                if resolution.height > 4000 {
                    2.0
                } else {
                    1.0
                }
            });
            placed.add(absolute::Output {
                port: screen.port,
                cfg: absolute::OutputConfig {
                    bounds,
                    scale,
                    resolution: Some(resolution),
                    active: true,
                },
            });
        }

        Ok(placed)
    }
}
