 
system TemplateLiteralsNested {
    interface:
        start()

    machine:
        $Init {
            start() {
                const inner = `b ${ 1 + 2 }`;
                const outer = `a ${ inner } c`;
                const nested = `x ${ `y ${ 3 * 4 } z` } w`;
                console.log(outer);
                console.log(nested);
                return
            }
        }
    actions:
    domain:
}
