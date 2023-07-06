using System.IO;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;
#nullable disable
namespace Basic
{
    
class BasicController : Basic
{
    public BasicController() : base() { }

    public new void entered_do(string msg)
    {
        entry_log.Add(msg);
    }

    public new void left_do(string msg)
    {
        exit_log.Add(msg);
    }

}
[TestFixture]
public class BasicTest
{
    [Test]
    public void InitialEnterEvent()
    {
        BasicController sm = new BasicController();
        CollectionAssert.AreEqual(
            sm.entry_log,
            new List<string> { "S0" },
            "Enter event is sent for entering the initial state on startup"
        );
    }

    [Test]
    public void TransitionEnterEvents()
    {
        BasicController sm = new BasicController();
        sm.entry_log = new List<string> {};
        sm.A();
        sm.B();
        CollectionAssert.AreEqual(
            sm.entry_log,
            new List<string> { "S1", "S0" },
            "Enter events are sent to the new state on transition"
        );
    }

    [Test]
    public void TransitionExitEvents()
    {
        var sm = new BasicController();
        sm.A();
        sm.B();
        CollectionAssert.AreEqual(
            sm.exit_log,
            new List<string> { "S0", "S1" },
            "Exit events are sent to the old state on transition"
        );
    }

    [Test]
    public void CurrentState()
    {
        BasicController sm = new BasicController();
        Assert.AreEqual(sm.state_info(), "0");
        sm.A();
        Assert.AreEqual(sm.state_info(), "1");
        sm.B();
        Assert.AreEqual(sm.state_info(), "0");
    }
}

}