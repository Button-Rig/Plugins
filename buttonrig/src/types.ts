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

export type TxPayload = SaveHandlerArgsPayload | ErrorPayload;

export function newSaveHandlerArgs(
  handlerArgs: string[]
): SaveHandlerArgsPayload {
  return {
    saveHandlerArgs: {
      handlerArgs,
    },
  };
}

export type RxPayload = LoadHandlerArgsPayload | null;

export class LoadHandlerArgsPayload {
    loadHandlerArgs: {
        handlerArgs: string[]
    }

    constructor(handlerArgs: string[]) {
        this.loadHandlerArgs = {
            handlerArgs
        }
    }
}

export interface SaveHandlerArgsPayload {
  saveHandlerArgs: {
    handlerArgs: string[];
  };
}
