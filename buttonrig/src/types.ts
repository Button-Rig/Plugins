export class ErrorPayload {
  error: {
    message: string | null;
  };

  constructor(message: string | null) {
    this.error = {
      message,
    };
  }
}

export interface Message {
  buttonPluginActionId: string;
  payload: TxPayload;
}

export function newMessage(payload: TxPayload): Message {
  return {
    buttonPluginActionId: window.name,
    payload: payload,
  };
}

export type TxPayload = TxReadyToReceive | TxSaveHandlerArgsPayload | ErrorPayload;

export function newSaveHandlerArgs(
  handlerArgs: string[]
): TxSaveHandlerArgsPayload {
  return {
    saveHandlerArgs: {
      handlerArgs,
    },
  };
}

export type RxPayload =  RxLoadHandlerArgsPayload | null;

export class RxLoadHandlerArgsPayload {
    loadHandlerArgs: {
        handlerArgs: string[]
    }

    constructor(handlerArgs: string[]) {
        this.loadHandlerArgs = {
            handlerArgs
        }
    }
}

export interface TxSaveHandlerArgsPayload {
  saveHandlerArgs: {
    handlerArgs: string[];
  };
}

export type TxReadyToReceive = "readyToReceive";
