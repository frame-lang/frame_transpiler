system TypeTest {
    interface:
        test_method(data: string)
        
    machine:
        $Start {
            test_method(data: string) {
                print(data)
            }
        }
}