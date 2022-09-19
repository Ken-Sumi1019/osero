use super::*;

#[test]
fn initializer_create_8_x_8_vec() {
    let bord = bord::build_bord();
    assert_eq!(bord.list.len(), 8);
    for i in 0..8 {
        assert_eq!(bord.list.get(i).unwrap().len(), 8);
    }
}

#[test]
fn first_4_discs_is_placed() {
    let bord = bord::build_bord();
    assert_eq!(bord.get_as_name('d', 4).is_on_disc(), true);
    assert_eq!(bord.get_as_name('e', 4).is_on_disc(), true);
    assert_eq!(bord.get_as_name('d', 5).is_on_disc(), true);
    assert_eq!(bord.get_as_name('e', 5).is_on_disc(), true);
    let mut count = 0;
    for i in 0..8 {
        for j in 0..8 {
            if !bord.get(i, j).is_on_disc() {
                count += 1;
            }
        }
    }
    assert_eq!(count, 60);
}
