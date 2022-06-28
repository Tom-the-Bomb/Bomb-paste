try {
    var hasEditor = true;
    var editor = ace.edit('content');
} catch {
    var hasEditor = false;
}

async function main() {

    if (hasEditor) {
        editor.setTheme('ace/theme/dracula');
        editor.setShowPrintMargin(false);

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
    }

    addLanguagesSelect();

    let saveButton = document.getElementById('saveButton');
    let editButton = document.getElementById('editButton');
    let newButton = document.getElementById('newButton')

    if (saveButton) {
        saveButton.addEventListener('click', async () => {
            const value = editor.getValue();

            let id = await makePostRequest(value);
            if (id !== null) {
                window.location.href = '/' + id;
            }
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
            sessionStorage.clear();
            window.location.href = '/';
        });
    }

    if (document.addEventListener) {
        document.addEventListener('keydown', (event) => {
            if ((event.key.toLowerCase() === 's' || event.keyCode === 83) && (event.metaKey || event.ctrlKey)) {
                event.preventDefault();
                saveButton.click();
            }
        }, false)
    }
}

function addLanguagesSelect() {
    let selectDiv = document.getElementById('language-select-div');

    if (selectDiv) {
        let innerHTML = '<select class="custom-select" id="language-select">\n';

        for (let lang in supportedAceLanguages) {
            lang = supportedAceLanguages[lang];
            innerHTML += `<option value="${lang.ace}">${lang.name}</option>\n`
        }
        innerHTML += '</select>'

        selectDiv.innerHTML = innerHTML;
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

    if (resp.status >= 400) {
        return null;
    } else {
        let json = await resp.json();
        return json.id;
    }
}

main();