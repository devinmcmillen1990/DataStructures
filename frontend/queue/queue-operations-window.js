class QueueOperationsWindow extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.version = 0;
        this.history = [];
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const templateHtml = await fetch('./queue/queue-operations-window.html').then(res => res.text());
        const style = `<link rel="stylesheet" href="./queue/queue-operations-window.css" />`;
        this.shadowRoot.innerHTML = style + templateHtml;
    }

    setCurrentState(state) {
        const output = this.shadowRoot.getElementById('output');
        if (output) {
            output.textContent = JSON.stringify(state, null, 2);
        }
    }

    addEntry(label, state) {
        this.version++;
        const versionCounter = this.shadowRoot.getElementById('versionCounter');
        if (versionCounter) {
            versionCounter.textContent = this.version;
        }

        this.history.push({ label, state });
        this.renderHistory();
    }

    renderHistory() {
        const historyDiv = this.shadowRoot.getElementById('history');
        if (!historyDiv) return;

        historyDiv.innerHTML = '';
        [...this.history].reverse().forEach((entry, index) => {
            const div = document.createElement('div');
            div.className = 'queue-ops-entry';
            div.innerHTML = `
          <div class="queue-ops-label">${this.version - index}. ${entry.label}</div>
          <code class="queue-ops-json">${JSON.stringify(entry.state)}</code>
        `;
            historyDiv.appendChild(div);
        });
    }
}

customElements.define('queue-operations-window', QueueOperationsWindow);
