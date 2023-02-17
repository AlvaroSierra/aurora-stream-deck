
let settings = {};

let websocket = undefined;

let saveSettings = (context, settings) => {
    var json = {
        "event": "setSettings",
        "context": context,
        "payload": settings
    };

    websocket.send(JSON.stringify(json));
};

function connectElgatoStreamDeckSocket(inPort, inUUID, inRegisterEvent, inInfo, inActionInfo) {

    const actionInfo = JSON.parse(inActionInfo);
    settings = actionInfo['payload']['settings']

    const action = actionInfo['action'];

    let socket = new WebSocket("ws://127.0.0.1:" + inPort);

    socket.onopen = function () {
        const json = {
            "event": inRegisterEvent,
            "uuid": inUUID
        }

        socket.send(JSON.stringify(json))
    };

    websocket = socket;

    if (action === "com.alvaro.aurorastream.zoomtonavid"){
        zoomtonavaid(inUUID)
    }
    else if (action === "com.alvaro.aurorastream.transfertostation"){
        transfertostation(inUUID)
    }
    else if (action === "com.alvaro.aurorastream.intercomreject"){
        intercomreject(inUUID)
    }
    else if (action === "com.alvaro.aurorastream.intercomaccept"){
        intercomaccept(inUUID)
    }
    else if (action === "com.alvaro.aurorastream.intercomcall"){
        intercomcall(inUUID)
    }

}

