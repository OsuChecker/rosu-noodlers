use rosu_map::Beatmap;
use rosu_map::section::hit_objects::HitObjectKind;
use rosu_noodlers::ln::{full_ln, full_ln_beat_divisor};
use rosu_noodlers::noln::noln;

fn setup() -> Beatmap {
    Beatmap::from_path("./resources/test.osu").expect("Failed to load test beatmap")
}

fn setup_full_ln() -> Beatmap {
    Beatmap::from_path("./resources/test2.osu").expect("Failed to load test beatmap")
}

#[test]
fn test_noln() {
    let mut map = setup();

    let holds_before = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Hold(_)))
        .count();

    assert!(holds_before > 0, "Test films should contain holds");

    noln(&mut map);

    let holds_after = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Hold(_)))
        .count();

    assert_eq!(holds_after, 0, "Test files should not contain holds anymore");
    map.encode_to_path("./resources/testnoln.osu");
}

#[test]
fn test_all_ln() {
    let mut map = setup_full_ln();

    let original_version = map.version.clone();

    let circles_before = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Circle(_)))
        .count();

    assert!(circles_before > 0, "Le fichier de test devrait contenir au moins un circle");

    full_ln(&mut map, 40.0, 30f64);

    let holds_after = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Hold(_)))
        .count();

    assert!(holds_after > 0, "Des circles devraient être transformés en holds");

    let mut last_end_by_column: Vec<f64> = vec![0.0; 4];
    for obj in map.hit_objects.iter() {
        if let HitObjectKind::Hold(hold) = &obj.kind {
            let column = (hold.pos_x / 512.0 * 4.0).floor() as usize;
            assert!(obj.start_time >= last_end_by_column[column],
                    "Les holds ne devraient pas se chevaucher dans la colonne {}", column);
            last_end_by_column[column] = obj.start_time + hold.duration;
        }
    }

    assert_eq!(map.version, format!("{} Full LN", original_version));
    map.encode_to_path("./resources/testfullln.osu");
}

#[test]
fn test_all_ln_beat_divisor() {
    let mut map = setup_full_ln();

    let original_version = map.version.clone();

    let circles_before = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Circle(_)))
        .count();

    assert!(circles_before > 0, "Le fichier de test devrait contenir au moins un circle");

    let beat_divisor : f64 = 1f64/8f64;
    full_ln_beat_divisor(&mut map, beat_divisor, Some(0f64));

    let holds_after = map.hit_objects.iter()
        .filter(|obj| matches!(obj.kind, HitObjectKind::Hold(_)))
        .count();

    assert!(holds_after > 0, "Des circles devraient être transformés en holds");

    let mut last_end_by_column: Vec<f64> = vec![0.0; 4];
    for obj in map.hit_objects.iter() {
        if let HitObjectKind::Hold(hold) = &obj.kind {
            let column = (hold.pos_x / 512.0 * 4.0).floor() as usize;
            assert!(obj.start_time >= last_end_by_column[column],
                    "Les holds ne devraient pas se chevaucher dans la colonne {}", column);
            last_end_by_column[column] = obj.start_time + hold.duration;
        }
    }

    assert_eq!(map.version, format!("{} Full LN", original_version));
    map.encode_to_path("./resources/testfulllnbeatdivisor.osu");
}