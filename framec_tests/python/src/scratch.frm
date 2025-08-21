system A {


    actions:

        a() {

            var x = 0
            var a = 0

            if x: y() elif a: b() else: c()
            if (x): y() elif (a): b() else: c()
            if x {
                y() y()
            } elif a {
                b() b()
            } else {
                c() c()
            }
                  
        }

}
