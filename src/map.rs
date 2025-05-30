use macroquad::prelude::Texture2D;

pub fn create_map(grass: &Texture2D) -> Vec<Vec<Option<Texture2D>>> {
    vec![
        vec![None; 25],
        vec![None; 25],
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[10] = Some(grass.clone());
            row[11] = Some(grass.clone());
            row[12] = Some(grass.clone());
            row
        },
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            row[3] = Some(grass.clone());
            row[4] = Some(grass.clone());
            row[6] = Some(grass.clone());
            row[7] = Some(grass.clone());
            row
        },
        vec![None; 25],
        {
            let mut row = vec![None; 25];
            for i in 8..15 {
                if i != 12 {
                    row[i] = Some(grass.clone());
                }
            }
            row
        },
        {
            let mut row = vec![None; 25];
            row[2] = Some(grass.clone());
            row[17] = Some(grass.clone());
            row[22] = Some(grass.clone());
            row
        },
        vec![Some(grass.clone()); 25],
    ]
}