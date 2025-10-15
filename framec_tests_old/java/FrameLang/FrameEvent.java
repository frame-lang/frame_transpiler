package framec_tests.java.FrameLang;

import java.util.*;

public class FrameEvent {
    public FrameEvent(String message, Map<String, Object> parameters) {
        this._message = message;
        this._parameters = parameters;
        this._return = null;
        return;
    }

    public FrameEvent() {
    }

    public String _message;
    public Map<String, Object> _parameters;
    public Object _return;


}
