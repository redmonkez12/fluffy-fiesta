use macroquad::prelude::Texture2D;

pub fn create_map(grass: &Texture2D) -> Vec<Vec<Option<Texture2D>>> {
    let mut map = Vec::new();

    for row_idx in 0..20 {
        let mut row = vec![None; 50];

        row[0] = Some(grass.clone());
        row[49] = Some(grass.clone());

        match row_idx {
            13 => {
                for i in 10..13 {
                    row[i] = Some(grass.clone());
                }
                for i in 44..47 {
                    row[i] = Some(grass.clone());
                }
            }
            15 => {
                for i in [3,4,6,7] {
                    row[i] = Some(grass.clone());
                }
                for i in 31..34 {
                    row[i] = Some(grass.clone());
                }
            }
            16 => {
                for i in 8..15 {
                    if i != 12 {
                        row[i] = Some(grass.clone());
                    }
                }
            }
            17 => {
                row[2] = Some(grass.clone());
                row[17] = Some(grass.clone());
                row[22] = Some(grass.clone());
            }
            18 => {
                for i in 20..28 {
                    row[i] = Some(grass.clone());
                }
            }
            19 => {
                for i in 0..50 {
                    row[i] = Some(grass.clone());
                }
            }
            _ => {}
        }

        map.push(row);
    }

    map
}
