function FrameEvent(message, parameters) {

    var that = {};

    that._message = message;
    that._parameters = parameters;
    that._return = null;

    return that;
}

module.exports = FrameEvent
