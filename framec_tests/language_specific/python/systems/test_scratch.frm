# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system A {


    actions:

        a() {

            x = 0
            a = 0

            if x: y() elif a: b() else: c()
            if (x): y() elif (a): b() else: c()
            if x:
                y() y()
            elif a:
                b() b()
            else:
                c() c()
            }
                  
        }

}
