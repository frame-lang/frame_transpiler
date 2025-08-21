fn main() {
    var service = StaticTestService()
}

system StaticTestService {

    operations:
        @staticmethod
        static_op() {
            print("static operation")
        }
        
        member_op() {
            print("member operation")
            self.static_op()  // Should generate static_op() without self.
        }

    machine:

    $Start {
        $>() {
            self.member_op()  // Should generate self.member_op()
            -> $Done
        }
    }

    $Done {
        $>() {
            print("Done")
        }
    }
}