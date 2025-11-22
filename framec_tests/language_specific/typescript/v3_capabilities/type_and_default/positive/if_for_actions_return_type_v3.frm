@target typescript

# V3 capability / regression guard for Bug 087:
# Ensure actions with explicit return types and native `if`/`for` bodies
# do not produce bogus methods like `public if(...)` or `public for(...)`.

system Bug087IfForActionsTs {

  interface:
    run()

  machine:
    $Idle {
      run() {
        this.spawnFramePython();
      }
    }

  actions:
    spawnFramePython(): boolean {
      const result = { success: true };
      if (result.success) {
        return true;
      }
      for (const x of [1, 2, 3]) {
        console.log(x);
      }
      return false;
    }
}

