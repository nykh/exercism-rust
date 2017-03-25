fn bottle(i: u64) -> String {
    if i == 0 {
        String::from("No more bottles of beer")
    } else if i == 1 {
        String::from("1 bottle of beer")
    } else {
        format!("{} bottles of beer", i)
    }
}

pub fn verse(i: u64) -> String {
    if i == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if i == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    }

    let j = i - 1;
    format!("{0} on the wall, {0}.\nTake one down and pass it around, {1} on the wall.\n",
            bottle(i), bottle(j).to_lowercase()
    )
}

pub fn sing(end: u64, start: u64) -> String {
    assert!(start < end);
    let mut song = verse(end);
    for i in (start..end).rev() {
        song.push('\n');
        song = song + verse(i).as_str();
    }

    song
}
