fn nand(a: i8, b: i8) -> i8 {
    if a == 0 && b == 0 {
        0
    } else {
        1
    }
}

fn not(in: i8) {
    if(in == 0) {
        1
    } else {
        0
    }
}

fn and(a: i8, b:i8)-> i8 {
    if a == 0 && b == 0 {
        1
    } else {
        0
    }
}

fn or(a: i8, b:i8) -> i8 {
    if a == 0 && b == 0 {
        0
    } else {
        1
    }
}

fn xor(a: i8, b: i8) -> i8 {
    if a != b {
        0
    } else {
        1
    }
}

fn mux(a: i8, b:i8, sel: i8) -> i8 {
    if sel == 0 {
        a
    } else {
        b
    }
}

fn dmux(in: i8, sel: i8) -> (i8,i8) {
    if sel == 0 {
        (in, 0)
    } else {
        (0,in)
    }
}
