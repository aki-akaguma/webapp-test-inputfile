function setDataUrlToBlobLink(base64DataUrl, mimeType, fileName, linkId) {
    if (base64DataUrl == '') { return; }

    // 1. Extract only pure data part from Base64
    const byteCharacters = atob(base64DataUrl.split(',')[1]);
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
// dummy call for minify
setDataUrlToBlobLink('','','','');

function getAnchorsDownloadHref(htmlId) {
    if (htmlId == '') { return; }
    const elm = document.getElementById(htmlId);
    if (elm) {
        return {download: elm.download, href: elm.href};
    }
    return {download: '', href: ''};
}
getAnchorsDownloadHref('');

function parseBlobData_dxsend(url) {
    if (url == '') { return; }
    let xhr=new XMLHttpRequest();
    xhr.open('GET',url,true);
    xhr.responseType='blob';
    xhr.onload=function(e) {
        let reader=new FileReader();
        reader.readAsDataURL(this.response);
        reader.onloaded=function(){
            let base64data=reader.result;
            dioxus.send(base64data);
        }
    }
}
parseBlobData('');
