class QueueOperationsWindow extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.version = 0;
        this.history = [];
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/queue-operations-window.html').then(res => res.text()),
            fetch('./index.css').then(res => res.text())
        ]);

        const style = document.createElement('style');
        style.textContent = css;

        const wrapper = document.createElement('div');
        wrapper.innerHTML = html;

        this.shadowRoot.innerHTML = '';
        this.shadowRoot.appendChild(style);
        this.shadowRoot.appendChild(wrapper);

        const clearBtn = this.shadowRoot.getElementById('clearHistoryBtn');
        if (clearBtn) {
            clearBtn.addEventListener('click', () => this.clearHistory());
        }
    }

    setCurrentState(state) {
        // Optional live state viewer
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

    clearHistory() {
        this.history = [];
        this.version = 0;
        const versionCounter = this.shadowRoot.getElementById('versionCounter');
        if (versionCounter) versionCounter.textContent = '0';
        this.renderHistory();
    }

    renderHistory() {
        const historyDiv = this.shadowRoot.getElementById('history');
        if (!historyDiv) return;

        historyDiv.innerHTML = '';
        [...this.history].reverse().forEach((entry, index) => {
            const div = document.createElement('div');
            div.className = 'border border-gray-200 rounded p-2 bg-gray-50';

            div.innerHTML = `
          <div class="text-sm font-semibold text-gray-700">${this.version - index}. ${entry.label}</div>
          <code class="block text-xs text-gray-600 bg-white mt-1 p-1 rounded overflow-x-auto">
            ${JSON.stringify(entry.state)}
          </code>
        `;

            historyDiv.appendChild(div);
        });
    }
}

customElements.define('queue-operations-window', QueueOperationsWindow);
