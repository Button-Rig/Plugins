import { newMessage, type RxPayload, type TxPayload, ErrorPayload, newSaveHandlerArgs, RxLoadHandlerArgsPayload } from "./types.js";

export function saveHandlerArgs(retriveArgsFn: () => string[] | ErrorPayload) {
    addEventListener("saveHandlerArgs", (_) => {
        let txPayload: TxPayload;
        let result = retriveArgsFn();
        if (result instanceof ErrorPayload) {
            txPayload = result;
        } else {
            txPayload = newSaveHandlerArgs(result);
        }
        postMessage(txPayload);
    })
}

export function loadHandlerArgs(fn: (handlerArgs: string[]) => void) {
    addEventListener("loadHandlerArgs", (payload) => {
        let p = payload as RxLoadHandlerArgsPayload;
        fn(p.loadHandlerArgs.handlerArgs);
    });
    postMessage("readyToReceive");
}

export function addEventListener(eventType: string, fn: (payload: RxPayload) => void)  {
    window.addEventListener("message", (event) => {
        if (!(event.data.event == eventType || Object.keys(event.data.event)[0] == eventType)) {
            return;
        }
        fn(event.data.event as RxPayload);
    })
}


export function postMessage(payload: TxPayload) {
    window.parent.postMessage(newMessage(payload), "*")
}