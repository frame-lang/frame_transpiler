using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

#nullable disable
namespace FrameLang
{
    public class FrameEvent
    {
    public string _message { get; set; }
    public Dictionary<string, object> _parameters { get; set; }
    public object _return { get; set; }

    public FrameEvent(string message, Dictionary<string, object> parameters)
    {
    
        _message = message;
        _parameters = parameters;
        
    }
    public FrameEvent(){}
    }
}