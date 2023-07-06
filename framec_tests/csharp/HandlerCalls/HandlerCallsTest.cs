using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace HandlerCalls
{
    class HandlerCallsController : HandlerCalls
    {
        public HandlerCallsController() : base()
        {
        }
        public void log_do(string from, int val)
        {
            string value = from + "(" + val.ToString() + ")";
            this.tape.Add(value);

        }
    }

    [TestFixture]
    public class HandlerCallsTest
    {
    [Test]
    public void TestCallsTerminateHandler() {
        var sm = new HandlerCallsController();
        sm.NonRec();
        sm.Foo(10);
        Assert.That(sm.tape.Contains("Unreachable(0)"), Is.False, "Handler calls unreachable statement");
    }

    [Test]
    public void TestNonRecursive() {
        var sm = new HandlerCallsController();
        sm.NonRec();
        sm.Foo(10);
        Assert.That(sm.tape, Is.EquivalentTo(new [] { "Foo(10)", "Bar(20)", "Final(30)" }));
    }

    [Test]
    public void TestSelfRecursive() {
        var sm = new HandlerCallsController();
        sm.SelfRec();
        sm.Foo(10);
        Assert.That(sm.tape, Is.EquivalentTo(new [] { "Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)" }));
    }

    [Test]
    public void TestMutuallyRecursive() {
        var sm = new HandlerCallsController();
        sm.MutRec();
        sm.Foo(2);
        Assert.That(sm.tape, Is.EquivalentTo(new [] {
            "Foo(2)",
            "Bar(4)",
            "Foo(4)",
            "Bar(8)",
            "Foo(16)",
            "Bar(32)",
            "Foo(96)",
            "Final(162)"
        }));
    }

    [Test]
    public void TestStringMatchCalls() {
        var sm = new HandlerCallsController();
        sm.NonRec();
        sm.Call("Foo", 5);
        Assert.That(sm.tape, Is.EquivalentTo(new [] { "Foo(5)", "Bar(10)", "Final(15)" }));
        sm.tape.Clear();

        sm.NonRec();
        sm.Call("Bar", 20);
        Assert.That(sm.tape, Is.EquivalentTo(new [] { "Bar(20)", "Final(20)" }));
        sm.tape.Clear();

        sm.NonRec();
        sm.Call("Qux", 37);
        Assert.That(sm.tape, Is.EquivalentTo(new [] { "Foo(1000)", "Bar(2000)", "Final(3000)" }));
    }
}

}
