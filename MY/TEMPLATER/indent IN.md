<%*
const direction = -1;
// 1: Promote headings (make them bigger)
// -1: Demote headings (make smaller)

const editor = app.workspace.activeLeaf.view.editor;
let selection = editor.getSelection();
if (selection=="") {
	// if there is no selection, set it to the current line instead
	let l = editor.getCursor().line;
	editor.setSelection({line: l, ch: 0}, {line: l, ch: 99});
	selection = editor.getSelection();
}

/* Because this script is being called from Templater, the selection is automatically replaced with the text of this script before being executed and consumed.  The next three lines reset the selection to what it was before execution. For some reason a timer of 0ms is sufficient for that. */
let cAnchor = editor.getCursor('anchor');
let cHead = editor.getCursor('head');
setTimeout(() => editor.setSelection(cAnchor, cHead), 0);

if (direction == 1)
	editor.replaceSelection(selection.replace(/^#(#+) /gm, "$1 "));
else if (direction == -1)
	editor.replaceSelection(selection.replace(/^#+ /gm, "#$&"));

%>
