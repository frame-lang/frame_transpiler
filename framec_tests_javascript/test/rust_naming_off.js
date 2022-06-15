const { describe, it } = require("mocha");
const assert = require("assert");
const RustNaming = require("../output/rust_naming_off");

class RustNamingController extends RustNaming {
  constructor() {
    super();
  }
  snake_action_do(snake_param) {
    this.snake_log.push(snake_param);
  }
  CamelAction_do(CamelParam) {
    this.CamelLog.push(CamelParam);
  }
  action123_do(param123) {
    this.log123.push(param123);
  }
  logFinal_do(r) {
    this.finalLog.push(r);
  }
}

describe("Rust naming off", () => {
  /// Test that the generated state machine works and that events are
  /// named as expected.
  it("Follow rust naming works", () => {
    let sm = new RustNamingController();

    sm.snake_event(1);
    sm.snake_event(2);
    sm.snake_event(1);
    sm.CamelEvent(3);
    sm.snake_event(1);
    sm.event123(4);
    assert.deepStrictEqual(sm.finalLog, [1103, 1104, 1105]);
    sm.finalLog = [];

    sm.CamelEvent(11);
    sm.snake_event(2);
    sm.CamelEvent(11);
    sm.CamelEvent(3);
    sm.CamelEvent(11);
    sm.event123(4);
    assert.deepStrictEqual(sm.finalLog, [1213, 1214, 1215]);
    sm.finalLog = [];

    sm.event123(21);
    sm.snake_event(2);
    sm.event123(21);
    sm.CamelEvent(3);
    sm.event123(21);
    sm.event123(4);
    assert.deepStrictEqual(sm.finalLog, [1323, 1324, 1325]);

    assert.deepStrictEqual(sm.snake_log, [1103, 1213, 1323]);
    assert.deepStrictEqual(sm.CamelLog, [1104, 1214, 1324]);
    assert.deepStrictEqual(sm.log123, [1105, 1215, 1325]);
  });

  /// Test that dynamic interface calls are renamed correctly.
  it("Interface calls", () => {
    let sm = new RustNamingController();
    sm.call("snake_event", 1);
    sm.call("CamelEvent", 2);
    sm.call("event123", 3);
    sm.call("snake_event", 4);
    sm.call("CamelEvent", 5);
    sm.call("event123", 6);
    assert.deepStrictEqual(sm.finalLog, [1103, 1307, 1211]);
    assert.deepStrictEqual(sm.snake_log, [1307]);
    assert.deepStrictEqual(sm.CamelLog, [1103]);
    assert.deepStrictEqual(sm.log123, [1211]);
  });
});
