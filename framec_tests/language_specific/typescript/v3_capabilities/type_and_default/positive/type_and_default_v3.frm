@target typescript

# V3 capability fixture: header type/default segment (`: Type = default`) in TypeScript.

system TypeAndDefaultDemoTs {
    operations:
        @native
        helper(x: number): Result = 0 {
            return x;
        }

    interface:
        compute(x: number, y: number): Result = 0

    machine:
        $A {
            e() {
                this.log("ok");
                this._operation_helper(1);
            }
        }

    actions:
        log(message: string): Result = 0 {
            console.log(message);
        }
}
