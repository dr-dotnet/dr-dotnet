async function downloadFileFromStream(fileName, contentStreamReference) {
    const arrayBuffer = await contentStreamReference.arrayBuffer();
    const blob = new Blob([arrayBuffer]);
    const url = URL.createObjectURL(blob);
    const anchorElement = document.createElement('a');
    anchorElement.href = url;
    anchorElement.download = fileName ?? '';
    anchorElement.click();
    anchorElement.remove();
    URL.revokeObjectURL(url);
}

function scrollToBottom(elementId) {
    const element = document.getElementById(elementId);
    if (!element) {
        return;
    }
    element.scrollTop = element.scrollHeight;
}

// Reports are transferred as streams rather than being embedded in the Blazor render tree, which
// would mean holding their whole content in memory server side and pushing it as a single render
// batch. This way the content is read from disk and sent to the browser in chunks.
async function setElementHtmlFromStream(elementId, contentStreamReference) {
    const element = document.getElementById(elementId);
    if (!element) {
        return;
    }
    const arrayBuffer = await contentStreamReference.arrayBuffer();
    element.innerHTML = new TextDecoder().decode(arrayBuffer);
}
