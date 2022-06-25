var editor = ace.edit("content");

function main() {
    editor.setTheme("ace/theme/dracula");

    editor.setBehavioursEnabled(true);
    editor.session.setOptions({
        tabSize: 4,
        useSoftTabs: true,
    });

    editor.container.style.lineHeight = 2;
    editor.renderer.updateFontSize();

    let saveButton = document.getElementById("saveButton");
    let editButton = document.getElementById("editButton");

    let form = document.getElementById("pasteForm");

    if (saveButton) {
        saveButton.onClick = () => {
            form.submit();
        };
    }

    if (editButton) {
        editButton.onClick = () => {
            const previousContent = document.getElementById("content").value;

            window.location.href = "/";
            let content = document.getElementById("content");
            content.value = previousContent

            form.submit();
        };
    }
}

function setReadOnly() {
    editor.setOptions({
        readOnly: true,
        highlightActiveLine: false,
        highlightGutterLine: false,
    })
}

function highlightResult() {
    let element = document.getElementById("content")

    language = hljs.highlightAuto(element.value);
    editor.session.setMode(
        "ace/mode/" + language.language || language.secondBest
    );

    setReadOnly();
}

main();