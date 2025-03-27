fn main() {
    let lines = [" twelve drummers drumming,"," eleven pipers piping,"," ten lords a-leaping,"," nine ladies dancing,"," eight maids a-milking,"," seven swans a-swimming,"," six geese a-laying,"," five gold rings,"," four calling birds,"," Three French hens,"," Two turtle doves, And"];
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in 0..12 {
        let mut middle_part = String::new();
        for j in 0..i {
            middle_part = format!("{}{}",lines[10-j],middle_part);
        }
        println!("On the {} day of Christmas my true love sent to me{} a partridge in a pear-tree",ordinal[i], middle_part);
    }
}

//the poem can be divided into 4 parts

//|On The | |first| | day of Christmas my true love sent to me | | | | a partridge in a pear-tree|

//|On The | |second| | day of Christmas my true love sent to me | |two turtle-doves,| | a partridge in a pear-tree|

//|On The | |third| | day of Christmas my true love sent to me | |three fat hens, two turtle-doves,| | a partridge in a pear-tree|

//we can write the poem by changing only the middle bit
