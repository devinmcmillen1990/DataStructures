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

    addEntry(label, state) {
        this.version++;
        const versionCounter = this.shadowRoot.getElementById('versionCounter');
        if (versionCounter) versionCounter.textContent = this.version;

        const historyDiv = this.shadowRoot.getElementById('history');
        if (!historyDiv) return;

        const div = document.createElement('div');
        div.className = 'border border-gray-200 rounded p-2 bg-gray-50';
        div.innerHTML = `
        <div class="text-sm font-semibold text-gray-700">${this.version}. ${label}</div>
        <code class="block text-xs text-gray-600 bg-white mt-1 p-1 rounded overflow-x-auto">${JSON.stringify(state)}</code>
      `;

        historyDiv.prepend(div);
    }

    clearHistory() {
        const historyDiv = this.shadowRoot.getElementById('history');
        if (historyDiv) historyDiv.innerHTML = '';
        this.version = 0;
        const versionCounter = this.shadowRoot.getElementById('versionCounter');
        if (versionCounter) versionCounter.textContent = '0';
    }
}

customElements.define('skiplist-expiry-operations-window', SkiplistExpiryOperationsWindow);