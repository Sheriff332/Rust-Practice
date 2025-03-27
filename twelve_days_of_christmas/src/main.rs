fn main() {
    let lines = [" twelve drummers drumming,"," eleven pipers piping,"," ten lords a-leaping,"," nine ladies dancing,"," eight maids a-milking,"," seven swans a-swimming,"," six geese a-laying,"," five gold rings,"," four calling birds,"," Three French hens,"," Two turtle doves, And"];
    
    for i in 0..11 {
        let mut middle_part = String::new();
        for j in 0..i {
            middle_part = format!("{}{}",lines[10-j],middle_part);
        }
        println!("The first day of Christmas my true love sent to me{} a partridge in a pear-tree", middle_part);
    }
}

//the poem can be divided into 3 parts

//|The first day of Christmas my true love sent to me | | | | a partridge in a pear-tree|

//|The second day of Christmas my true love sent to me | |two turtle-doves,| | a partridge in a pear-tree|

//|The third day of Christmas my true love sent to me | |three fat hens, two turtle-doves,| | a partridge in a pear-tree|

//we can write the poem by changing only the middle bit
