var editor = ace.edit('content');

async function main() {
    editor.setTheme('ace/theme/dracula');

    editor.setBehavioursEnabled(true);
    editor.session.setOptions({
        tabSize: 4,
        useSoftTabs: true,
    });

    editor.container.style.lineHeight = 2;
    editor.renderer.updateFontSize();

    if (sessionStorage.getItem("previousContent")) {
        editor.setValue(sessionStorage.getItem("previousContent"), 1)
    }

    let saveButton = document.getElementById('saveButton');
    let editButton = document.getElementById('editButton');

    if (saveButton) {
        saveButton.addEventListener('click', async () => {
            const value = editor.getValue();

            let result = await makePostRequest(value);
            window.location.href = '/' + result.id;
        });
    }

    if (editButton) {
        editButton.addEventListener('click', () => {
            const previousContent = editor.getValue();
            sessionStorage.setItem("previousContent", previousContent);

            window.location.href = '/';
        });
    }
}

function highlightResult() {
    console.log(1)
    editor.setOptions({
        readOnly: true,
        highlightActiveLine: false,
        highlightGutterLine: false,
    });
    console.log(2)
    let element = document.getElementById('content');

    language = hljs.highlightAuto(element.value);
    editor.session.setMode(
        'ace/mode/' + language.language || language.secondBest
    );
}

async function makePostRequest(value) {
    let payload = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            'content': value,
        }),
    }

    let resp = await fetch('/upload', payload);
    return await resp.json();
}

main();