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

    if (sessionStorage.getItem('previousContent')) {
        editor.setValue(sessionStorage.getItem('previousContent'), 1);
        sessionStorage.removeItem('previousContent');
    }

    if (sessionStorage.getItem('previousLanguage')) {
        editor.session.setMode(
            'ace/mode/' + sessionStorage.getItem('previousLanguage'),
        );
        sessionStorage.removeItem('previousLanguage');
    }

    if (window.location.pathname.match(/\/[a-zA-Z0-9]{20}#?.*$/)) {
        highlightResult();
    }

    let saveButton = document.getElementById('saveButton');
    let editButton = document.getElementById('editButton');
    let newButton = document.getElementById('newButton')

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
            sessionStorage.setItem('previousContent', previousContent);

            window.location.href = '/';
        });
    }

    if (newButton) {
        newButton.addEventListener('click', () => {
            window.location.href = '/';
        });
    }
}

function highlightResult() {
    editor.setOptions({
        readOnly: true,
        highlightActiveLine: false,
        highlightGutterLine: false,
    });

    let value = editor.getValue();

    let language = hljs.highlightAuto(value);
    language = language.language || language.secondBest

    if (language) {
        editor.session.setMode(
            'ace/mode/' + language,
        );

        sessionStorage.setItem('previousLanguage', language)
    }
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
    };

    let resp = await fetch('/upload', payload);
    return await resp.json();
}

main();