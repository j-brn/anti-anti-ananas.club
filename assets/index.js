const updateMessage = (preambleContainerId, messageContainerId, url) => {
  const preambleContainer = document.getElementById(preambleContainerId);
  const messageContainer = document.getElementById(messageContainerId);

  fetch(url)
      .then(response => response.json())
      .then(data => {
        preambleContainer.textContent = data.preamble;
        messageContainer.textContent = data.message;
      })
      .catch(error => console.error(error))
};

document.addEventListener('DOMContentLoaded', (_event) => {
  const preambleBoxId = 'preamble';
  const messageBoxId = 'message';
  const messageEndpoint = '/api/message';
  const messageUpdateInterval = 4000;

  // set initial message
  updateMessage(preambleBoxId, messageBoxId, messageEndpoint);

  // change message every few seconds
  setInterval(() => {
    updateMessage(preambleBoxId, messageBoxId, messageEndpoint);
  }, messageUpdateInterval)
})