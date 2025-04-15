class SkiplistExpiryCommandRibbon extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./time-indexed/skiplist/skiplist-expiry-command-ribbon.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;

        const input = this.shadowRoot.getElementById('valueInput');
        const enqueueBtn = this.shadowRoot.getElementById('enqueueButton');
        const clearBtn = this.shadowRoot.getElementById('clearButton');

        enqueueBtn.addEventListener('click', () => {
            const val = input.value.trim();
            if (val !== '') {
                document.dispatchEvent(new CustomEvent('enqueue', { detail: val }));
                input.value = '';
            }
        });

        clearBtn.addEventListener('click', () => {
            document.dispatchEvent(new Event('clear-queue'));
        });
    }

    setQueueEmptyState(isEmpty) {
        const clearBtn = this.shadowRoot.getElementById('clearButton');
        if (clearBtn) clearBtn.disabled = isEmpty;
    }
}

customElements.define('skiplist-expiry-command-ribbon', SkiplistExpiryCommandRibbon);