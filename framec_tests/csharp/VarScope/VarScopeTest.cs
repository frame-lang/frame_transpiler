using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace VarScope
{
    class VarScopeController : VarScope
    {
        public VarScopeController() : base()
        {
        }
        public void log_do(string s)
        {
            this.tape.Add(s);
        }
        public void do_nn()
        {
            this.nn("|nn|[d]");
        }
        public void do_ny()
        {
            this.ny("|ny|[d]");
        }
        public void do_yn()
        {
            this.yn("|yn|[d]", "|yn|[x]");
        }
        public void do_yy()
        {
            this.yy("|yy|[d]", "|yy|[x]");
        }
    }

    [TestFixture]
    public class VarScopeTest
    {
        protected List<string> expected(string state, string msg, string x)
        {
            List<string> result = new List<string>();
            result.Add("#.a");
            result.Add("$" + state + "[b]");
            result.Add("$" + state + ".c");
            result.Add("|" + msg + "|" + "[d]");
            result.Add("|" + msg + "|" + ".e");
            result.Add(x);

            return result;
        }

        [Test]
        public void TestNoShadowing()
        {
            VarScopeController sm = new VarScopeController();
            sm.to_nn();
            sm.do_nn();
            Assert.AreEqual(expected("NN", "nn", "#.x"), sm.tape);
        }

        [Test]
        public void TestAllShadowingScenerios()
        {
            VarScopeController sm = new VarScopeController();
            sm.to_nn();
            sm.do_ny();
            Assert.AreEqual(expected("NN", "ny", "|ny|.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yn();
            Assert.AreEqual(expected("NN", "yn", "|yn|[x]"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yy();
            Assert.AreEqual(expected("NN", "yy", "|yy|.x"), sm.tape);

            sm = new VarScopeController();
            sm.to_ny();
            sm.do_nn();
            Assert.AreEqual(expected("NY", "nn", "$NY.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_ny();
            Assert.AreEqual(expected("NY", "ny", "|ny|.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yn();
            Assert.AreEqual(expected("NY", "yn", "|yn|[x]"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yy();
            Assert.AreEqual(expected("NY", "yy", "|yy|.x"), sm.tape);

            sm = new VarScopeController();
            sm.to_yn();
            sm.do_nn();
            Assert.AreEqual(expected("YN", "nn", "$YN[x]"), sm.tape);
            sm.tape = new List<string>();
            sm.do_ny();
            Assert.AreEqual(expected("YN", "ny", "|ny|.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yn();
            Assert.AreEqual(expected("YN", "yn", "|yn|[x]"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yy();
            Assert.AreEqual(expected("YN", "yy", "|yy|.x"), sm.tape);

            sm = new VarScopeController();
            sm.to_yy();
            sm.do_nn();
            Assert.AreEqual(expected("YY", "nn", "$YY.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_ny();
            Assert.AreEqual(expected("YY", "ny", "|ny|.x"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yn();
            Assert.AreEqual(expected("YY", "yn", "|yn|[x]"), sm.tape);
            sm.tape = new List<string>();
            sm.do_yy();
            Assert.AreEqual(expected("YY", "yy", "|yy|.x"), sm.tape);
        }
    }

}