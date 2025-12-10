@target rust

system S {
    machine:
        $A => $P {
            e() {
                #[derive(Debug)]
                struct Point { x: i32, y: i32 }
                let mut p = Point { x: 1, y: 2 };
                p.x += 3;
                p.y -= 1;
                let _sum = p.x + p.y;
                => $^;
            }
        }
        $P { }
}
