@@target python

@@system MultiTargetDemo {
    actions:
        log_all() {
            #[target: python]
            {
                print("python block reached")
            }

            #[target: c]
            {
                printf("c block reached\\n");
            }

            #[target: cpp]
            {
                std::cout << "cpp block reached" << std::endl;
            }

            #[target: java]
            {
                System.out.println("java block reached");
            }

            #[target: csharp]
            {
                Console.WriteLine("csharp block reached");
            }

            #[target: rust]
            {
                println!("rust block reached");
            }

            return
        }
}
