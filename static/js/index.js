ace.require('ace/ext/language_tools');

try {
    var hasEditor = true;
    var editor = ace.edit('content');
} catch {
    var hasEditor = false;
}

async function main() {

    if (hasEditor) {
        editor.setTheme('ace/theme/dracula');
        editor.renderer.setScrollMargin(20);
        editor.renderer.setPadding(8);
        editor.setShowPrintMargin(false);

        editor.setBehavioursEnabled(true);
        editor.setOptions({
            tabSize: 4,
            useSoftTabs: true,
            showLineNumbers: true,
            enableSnippets: true,
            enableBasicAutocompletion: true,
        });

        editor.container.style.lineHeight = 2;
        editor.renderer.updateFontSize();

        if (sessionStorage.getItem('previousContent')) {
            editor.setValue(sessionStorage.getItem('previousContent'), 1);
            sessionStorage.removeItem('previousContent');
        }

        if (sessionStorage.getItem('previousLanguage')) {
            editor.session.setMode(
                sessionStorage.getItem('previousLanguage'),
            );
        }

        let paste_route = window.location.pathname.match(/\/([a-zA-Z0-9]{20})(\.([^#]+))?(#?\S*)$/);
        if (paste_route) {
            let language = getClosestAceLanguage(paste_route[3], more=false);
            highlightResult(editor.getValue(), language);
        }
    }

    addLanguagesSelect();

    let helpButton = document.getElementById('helpButton');
    let saveButton = document.getElementById('saveButton');
    let editButton = document.getElementById('editButton');
    let newButton = document.getElementById('newButton');
    let copyButton = document.getElementById('copyButton');
    let languagesSelect = document.getElementById('language-select');

    if (helpButton) {
        helpButton.addEventListener('click', () => {
            window.location.href = '/help';
        })
    }

    if (saveButton) {
        saveButton.addEventListener('click', async () => {
            const value = editor.getValue();

            let json = await makePostRequest(value);
            if (json !== null) {
                language = highlightResult(value, apply=false);
                window.location.href = `/${json.id}.${language}`;
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

    if (copyButton) {
        copyButton.addEventListener('click', () => {
            const path = window.location.href;
            navigator.clipboard.writeText(path);
            alert('Copied URL to clipboard!');
        });
    }

    if (languagesSelect) {
        languagesSelect.addEventListener('change', () => {
            let selected = languagesSelect.options[languagesSelect.selectedIndex];

            if (hasEditor) {
                editor.session.setMode(selected.value);
            }
            sessionStorage.setItem('previousLanguage', selected.value);
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

    $(function () {
        $('[data-toggle="tooltip"]').tooltip();
    })
}

function getClosestAceLanguage(query, more=true) {

    if (query) {
        query = query.toLowerCase();

        if (query in supportedAceLanguages) {
            return query;
        }

        for (let lang in supportedAceLanguages) {
            langInfo = supportedAceLanguages[lang];

            let file_extensions = langInfo.extensions.split('|');

            if (more) {
                var condition = file_extensions.includes(query) || lang.includes(query) || langInfo.caption.includes(query);
            } else {
                var condition = file_extensions.includes(query);
            }

            if (condition) {
                return lang;
            }
        }
    }

    return query;
}

function addLanguagesSelect() {
    let selectDiv = document.getElementById('language-select-div');

    if (selectDiv) {
        if (hasEditor) {
            var currLang = sessionStorage.getItem('previousLanguage') || editor.session.getMode().$id;
        } else {
            var currLang = sessionStorage.getItem('previousLanguage');
        }

        let innerHTML = '<select class="custom-select" id="language-select">\n';

        for (let lang in supportedAceLanguages) {
            lang = supportedAceLanguages[lang];
            const langName = lang.name.replace('_', '-');

            if (lang.mode === currLang) {
                innerHTML += `<option value="${lang.mode}" selected>${langName}</option>\n`;
            } else {
                innerHTML += `<option value="${lang.mode}">${langName}</option>\n`;
            }
        }
        innerHTML += '</select>';

        selectDiv.innerHTML = innerHTML;
    }
}

function highlightResult(value, language=null, apply=true) {
    let resultLanguage = 'txt';

    if (apply) {
        editor.setOptions({
            readOnly: true,
            highlightActiveLine: false,
            highlightGutterLine: false,
        });
    }

    if (sessionStorage.getItem('previousLanguage') && !language) {
        language = sessionStorage.getItem('previousLanguage');
        resultLanguage = language.replace('ace/mode/', '');
    } else {
        if (!language) {
            language = hljs.highlightAuto(value);
            language = language.language || language.secondBest.language || 'text';
            language = getClosestAceLanguage(language);
        }

        const isValidLang = language.toLowerCase() in supportedAceLanguages;

        if (isValidLang) {
            resultLanguage = language;
        }

        language = 'ace/mode/' + language.toLowerCase();

        if (isValidLang) {
            sessionStorage.setItem('previousLanguage', language);
        }
    }

    if (apply) {
        editor.session.setMode(language);
    }

    return resultLanguage;
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

    try {
        let resp = await fetch('/upload', payload);

        if (resp.ok) {
            return await resp.json();
        } else {
            if ([400, 413, 429, 500].includes(resp.status)){
                alert(await resp.text());
            } else {
                alert(`${resp.status}: Something went wrong :(`);
            }

            return null;
        }
    } catch {
        alert('Something went wrong D:');
        return null;
    }
}

main();