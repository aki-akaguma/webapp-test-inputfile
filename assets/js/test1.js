function setDataToBlobLink(base64Data, fileName, mimeType, linkId) {
    // 1. Extract only pure data part from Base64
    const byteCharacters = atob(base64Data.split(',')[1]);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);

    // 2. Create Blob object
    const blob = new Blob([byteArray], { type: mimeType });

    // 3. Generate temporary URL (Object URL)
    const url = window.URL.createObjectURL(blob);

    // 4. Set to link object
    const link = document.getElementById(linkId);
    link.href = url;
    link.download = fileName;

    /*
    // 4. Create and click invisible link
    const link = document.createElement('a');
    link.href = url;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();

    // 5. Free memory
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
    */
}
