function saveHandlerArgs(retriveArgsFn) {
  window.addEventListener("message", (event) => {
    if (event.data.type == "saveHandlerArgs") {
      let retriveHandlerArgsResponse = retriveArgsFn();
      if (retriveHandlerArgsResponse.error) {
        
      }
      window.parent.postMessage(
        {
          buttonPluginActionId: window.name,
          payload: {
            saveHandlerArgs: {
              handlerArguments,
            },
          },
        },
        "*"
      );
    }
  });
}
