use rosu_map::Beatmap;
use rosu_map::section::hit_objects::{HitObjectCircle, HitObjectKind};
use rosu_map::util::Pos;

/// Converts all hold notes (LNs) in the beatmap to normal notes (circles).
///
/// # Arguments
/// * `map` - A mutable reference to the Beatmap to modify
///
/// # Example
/// ```
/// let mut map = Beatmap::from_path("map.osu").unwrap();
/// noln(&mut map);
/// ```

pub fn noln(map : &mut Beatmap) {

    for hit_object in map.hit_objects.iter_mut() {
        if let HitObjectKind::Hold(hold) = hit_object.kind {
            hit_object.kind = HitObjectKind::Circle(HitObjectCircle {
                pos: Pos {
                    x: hold.pos_x,
                    y: 0.0,
                },
                new_combo: false,
                combo_offset: 0,
            });
        }
    }

    map.version = format!("{} No LN", map.version);
}

