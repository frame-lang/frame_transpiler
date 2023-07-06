using System.Globalization;
using System.IO;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

#nullable disable
namespace StateParams
{
    class StateParamsController : StateParams
{
        public StateParamsController() : base()
        {
        }

    protected new void got_param_do(string name, int val) 
    {
        this.param_log.Add(name+"="+(val).ToString());
    }
}


[TestFixture]
public class StateParamsTest {
    [Test]
    public void TestSingleParameter(){
        StateParamsController sm = new StateParamsController();
        sm.Next();
        sm.Log();
        Assert.AreEqual(new List<string>{"val=1"}, sm.param_log);
    }

    [Test]
    public void TestMultipleParameters(){
        StateParamsController sm = new StateParamsController();
        sm.Next();
        sm.Next();
        sm.Log();
        Assert.AreEqual(new List<string>{"left=1", "right=2"}, sm.param_log);
    }

    [Test]
    public void TestSeveralPasses(){
        StateParamsController sm = new StateParamsController();

        sm.Next(); // val=1
        sm.Next(); // left=1 right=2
        sm.Next(); // val=3
        sm.Log();
        sm.Prev(); // left=4 right=3
        sm.Log();
        sm.Prev(); // val=12
        sm.Log();
        Assert.AreEqual(new List<string>{"val=3", "left=4", "right=3", "val=12"}, sm.param_log);
    }
}

}