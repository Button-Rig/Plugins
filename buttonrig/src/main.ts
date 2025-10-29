import { newMessage, type RxPayload, type TxPayload, ErrorPayload, newSaveHandlerArgs, LoadHandlerArgsPayload } from "./types.js";

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
        if (payload instanceof LoadHandlerArgsPayload) {
            fn(payload.loadHandlerArgs.handlerArgs);
        }
    });
}

function addEventListener(eventName: string, fn: (payload: RxPayload) => void) {
    window.addEventListener("message", (event) => {
        if (event.data.event != eventName) {
            return;
        }
        fn(event.data.payload as RxPayload);
    })
}

function postMessage(payload: TxPayload) {
    window.parent.postMessage(newMessage(payload), "*")
}