const { describe, it } = require("mocha");
const assert = require("assert");
const StateParams = require("../output/state_params");

class StateParamsController extends StateParams {
  constructor() {
    super();
  }
  got_param_do(name, val) {
    this.param_log.push(`${name}=${val}`);
  }
}

describe("State params", () => {
  it("Single parameter", () => {
    let sm = new StateParamsController();

    sm.Next();
    sm.Log();

    assert.deepStrictEqual(sm.param_log, ["val=1"]);
  });

  it("Multiple Parameters", ()=>{
      let sm = new StateParamsController()

      sm.Next()
      sm.Next()
      sm.Log()

      assert.deepStrictEqual(sm.param_log, ["left=1", "right=2"])
  })

  it("Several passes", ()=> {
    let sm = new StateParamsController()
    sm.Next(); // val=1
    sm.Next(); // left=1, right=2
    sm.Next(); // val=3
    sm.Log();
    sm.Prev(); // left=4, right=3
    sm.Log();
    sm.Prev(); // val=12
    sm.Log();
    assert.deepStrictEqual(sm.param_log, ["val=3", "left=4", "right=3", "val=12"]);
})
});
