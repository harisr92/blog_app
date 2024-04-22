class Editor {
  constructor(ele, options) {
    this.element = ele;
    this.textarea = ele.querySelector("textarea");
    this.options = { ...this.defaults(), ...options }
  }

  init() {
    this.editor = document.createElement("iframe");
    this.editor.style.background = "white";
    this.editor.style.color = "grey";
    this.editor.setAttribute("width", "100%")
    this.textarea.classList.add("hidden");

    this.textarea.parentNode.insertBefore(this.editor, this.textarea.nextSibiling);

    this.editor = this.element.querySelector("iframe");
    this.editor.srcdoc = this.textarea.innerText;
    this.editor.onload = this.options.actions.onloadfn(this)

    this.options.actions.bold(this)
    this._instanceId = this.element.id
  }

  defaults() {
    return {
      actions: {
        onloadfn: function (editor) {
          return(function () {
            editor.editor.contentWindow.document.designMode = "on";
            editor.editor.contentDocument.body.contentEditable = true;
            editor.editor.contentWindow.document.addEventListener("keyup", function () {
              editor.save()
            });
          });
        },
        bold: function (editor) {
          const boldBtn = editor.element.querySelector("#bold-btn");
          boldBtn.addEventListener("click", function () {
            editor.bold();
          });
        },
      }
    };
  }

  bold() {
    this.editor.contentWindow.document.execCommand("bold", false, null);
    this.save();
  }

  save() {
    this.textarea.innerText = this.editor.contentWindow.document.body.innerHTML;
    this.editor.contentWindow.document.body.focus();
  }
}

if (typeof window !== "undefined") {
  window.Editor = Editor;
}
