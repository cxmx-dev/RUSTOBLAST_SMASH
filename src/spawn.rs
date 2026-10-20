use crate::constants::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnKind {
    BigRock,
    SmallSpinner,
    BigSpinner,
    GuidedMissile,
    Ufo,
    PolyWave,
}

/// `roll` and `wave_roll` are independent 0..1 samples.
pub fn roll_spawn(score: i32, roll: f32, wave_roll: f32) -> SpawnKind {
    if wave_roll < POLY_WAVE_CHANCE {
        return SpawnKind::PolyWave;
    }
    if score >= UFO_UNLOCK_SCORE {
        if roll < LATE_UFO {
            SpawnKind::Ufo
        } else if roll < LATE_GUIDED {
            SpawnKind::GuidedMissile
        } else if roll < LATE_SMALL_SPINNER {
            SpawnKind::SmallSpinner
        } else if roll < LATE_BIG_SPINNER {
            SpawnKind::BigSpinner
        } else {
            SpawnKind::BigRock
        }
    } else if roll < EARLY_GUIDED {
        SpawnKind::GuidedMissile
    } else if roll < EARLY_SMALL_SPINNER {
        SpawnKind::SmallSpinner
    } else if roll < EARLY_BIG_SPINNER {
        SpawnKind::BigSpinner
    } else {
        SpawnKind::BigRock
    }
}

pub fn spawn_interval(multiplier: i32) -> f64 {
    let freq_mult = 1.0 + (multiplier as f64 - 1.0) * 0.05;
    1.0 / freq_mult
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sim_band(score: i32, seconds: f64) -> (u32, [u32; 6]) {
        let multiplier = if score > 100_000 {
            6
        } else if score > 50_000 {
            5
        } else if score > 20_000 {
            4
        } else if score > 5_000 {
            3
        } else if score > 1_000 {
            2
        } else {
            1
        };
        let interval = spawn_interval(multiplier);
        let ticks = (seconds / interval).floor() as u32;
        let mut counts = [0u32; 6];
        let mut waves = 0u32;
        // Deterministic LCG so the test is stable
        let mut seed: u32 = 1 + score as u32;
        let mut next = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            (seed as f32) / (u32::MAX as f32)
        };
        for _ in 0..ticks {
            match roll_spawn(score, next(), next()) {
                SpawnKind::BigRock => counts[0] += 1,
                SpawnKind::SmallSpinner => counts[1] += 1,
                SpawnKind::BigSpinner => counts[2] += 1,
                SpawnKind::GuidedMissile => counts[3] += 1,
                SpawnKind::Ufo => counts[4] += 1,
                SpawnKind::PolyWave => {
                    waves += 1;
                    counts[5] += POLY_WAVE_COUNT as u32;
                }
            }
        }
        (waves, counts)
    }

    #[test]
    fn five_min_mix_is_rock_led_not_ufo_led() {
        for score in [0, 5_000, 20_000] {
            let (waves, c) = sim_band(score, 300.0);
            let total_threats: u32 = c.iter().sum();
            let rocks = c[0];
            let ufos = c[4];
            assert!(total_threats > 200, "score {score}: too few spawns");
            assert!(
                rocks as f32 / total_threats as f32 > 0.35,
                "score {score}: rocks should lead, got {rocks}/{total_threats}"
            );
            if score < UFO_UNLOCK_SCORE {
                assert_eq!(ufos, 0, "UFOs before unlock");
            } else {
                let ufo_share = ufos as f32 / total_threats as f32;
                assert!(
                    ufo_share < 0.18,
                    "score {score}: UFO share {ufo_share} too high"
                );
            }
            assert!(waves < 20, "score {score}: too many poly waves ({waves})");
        }
    }
}
