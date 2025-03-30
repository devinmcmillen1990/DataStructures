class QueueCommandRibbon extends HTMLElement {
    constructor() {
      super();
      this.attachShadow({ mode: 'open' });
    }
  
    async connectedCallback() {
      const [html, css] = await Promise.all([
        fetch('./queue/queue-command-ribbon.html').then(res => res.text()),
        fetch('./queue/queue-command-ribbon.css').then(res => res.text())
      ]);
  
      this.shadowRoot.innerHTML = `
        <style>${css}</style>
        ${html}
      `;
  
      const input = this.shadowRoot.getElementById('valueInput');
      const enqueueBtn = this.shadowRoot.getElementById('enqueueBtn');
      const dequeueBtn = this.shadowRoot.getElementById('dequeueBtn');
  
      input.addEventListener('input', () => {
        enqueueBtn.disabled = input.value.trim() === "";
      });
  
      enqueueBtn.addEventListener('click', () => {
        const val = input.value.trim();
        if (val) {
          this.dispatchEvent(new CustomEvent('enqueue', { detail: val, bubbles: true }));
          input.value = "";
          enqueueBtn.disabled = true;
        }
      });
  
      dequeueBtn.addEventListener('click', () => {
        this.dispatchEvent(new Event('dequeue', { bubbles: true }));
      });
  
      // Local keybinding
      input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
          e.preventDefault();
          enqueueBtn.click();
        }
      });
  
      // Global keybindings
      window.addEventListener('keydown', (e) => {
        const isInputFocused = this.shadowRoot.contains(document.activeElement);
        const isInputEmpty = input.value.trim() === "";
  
        if ((e.key === 'Delete') && (isInputEmpty || !isInputFocused)) {
          e.preventDefault();
          dequeueBtn.click();
        }
      });
  
      this._input = input;
      this._dequeueBtn = dequeueBtn;
    }
  
    setQueueEmptyState(isEmpty) {
      if (this._dequeueBtn) {
        this._dequeueBtn.disabled = isEmpty;
      }
    }
  }
  
  customElements.define('queue-command-ribbon', QueueCommandRibbon);
  