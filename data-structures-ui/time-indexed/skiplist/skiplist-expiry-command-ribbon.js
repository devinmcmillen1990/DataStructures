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
        const ttlSelect = this.shadowRoot.getElementById('ttlSelect');
        const enqueueBtn = this.shadowRoot.getElementById('enqueueButton');
        const clearBtn = this.shadowRoot.getElementById('clearButton');

        const getTTL = () => parseInt(ttlSelect.value) * 1000;

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const value = input.value.trim();
                if (value) {
                    document.dispatchEvent(new CustomEvent('enqueue', { detail: { value, ttl: getTTL() } }));
                    input.value = '';
                }
            }
        });

        enqueueBtn.addEventListener('click', () => {
            const value = input.value.trim();
            if (value) {
                document.dispatchEvent(new CustomEvent('enqueue', { detail: { value, ttl: getTTL() } }));
                input.value = '';
            }
        });

        clearBtn.addEventListener('click', () => {
            document.dispatchEvent(new CustomEvent('clear-queue'));
        });
    }

    setQueueEmptyState(isEmpty) {
        const clearBtn = this.shadowRoot.getElementById('clearBtn');
        if (clearBtn) {
            clearBtn.disabled = isEmpty;
        }
    }
}

customElements.define('skiplist-expiry-command-ribbon', SkiplistExpiryCommandRibbon);
