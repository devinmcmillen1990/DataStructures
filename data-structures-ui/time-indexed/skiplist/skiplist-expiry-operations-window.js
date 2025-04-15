class SkiplistExpiryOperationsWindow extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.version = 0;
        this.history = [];
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./time-indexed/skiplist/skiplist-expiry-operations-window.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;
    }

    setCurrentState(state) {
        // Optionally handle live state updates elsewhere
    }

    addEntry(label, state, expiresAt = null) {
        this.version++;
        this.history.push({ label, state, expiresAt });
        this.renderHistory();
    }

    clearHistory() {
        this.history = [];
        this.version = 0;
        this.renderHistory();
    }

    renderHistory() {
        const historyDiv = this.shadowRoot.getElementById('history');
        if (!historyDiv) return;

        historyDiv.innerHTML = '';

        [...this.history].reverse().forEach((entry, index) => {
            const div = document.createElement('div');
            div.className = 'border border-gray-200 rounded p-2 bg-gray-50';

            const timestamp = entry.expiresAt
                ? `expires at ${entry.expiresAt.toLocaleTimeString()}`
                : new Date().toLocaleString();

            div.innerHTML = `
          <div class="text-sm font-semibold text-gray-700">${this.version - index}. ${entry.label}</div>
          <div class="text-xs text-gray-500">${timestamp}</div>
          <code class="block text-xs text-gray-600 bg-white mt-1 p-1 rounded overflow-x-auto">${JSON.stringify(entry.state)}</code>
        `;

            historyDiv.appendChild(div);
        });
    }
}

customElements.define('skiplist-expiry-operations-window', SkiplistExpiryOperationsWindow);
