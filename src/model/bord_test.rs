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

#[test]
fn test_put() {
    let mut bord = bord::build_bord();
    bord.put(2, 3, grid::Color::Black);
    assert_eq!(bord.get(2, 3).disc_color(), &grid::Color::Black);
}

#[test]
fn test_put_as_name() {
    let mut bord = bord::build_bord();
    bord.put_as_name('c', 4, grid::Color::Black);
    assert_eq!(bord.get(2, 3).disc_color(), &grid::Color::Black);
}

#[test]
fn test_get() {
    let bord = bord::build_bord();
    assert_eq!(bord.get(3, 3).disc_color(), &grid::Color::White);
}

#[test]
fn test_get_as_name() {
    let bord = bord::build_bord();
    assert_eq!(bord.get_as_name('d', 4).disc_color(), &grid::Color::White);
}
