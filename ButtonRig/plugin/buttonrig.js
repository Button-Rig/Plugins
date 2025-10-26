function saveHandlerArguments(handlerArguments) {
  window.parent.postMessage(
    {
      buttonPluginActionId: window.name,
      payload: {
        saveHandlerArguments: {
          handlerArguments,
        },
      },
    },
    "*"
  );
}
