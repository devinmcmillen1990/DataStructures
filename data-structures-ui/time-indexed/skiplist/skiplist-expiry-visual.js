class SkipListExpiryVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./time-indexed/skiplist/skiplist-expiry-visual.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;
    }

    update(nodes) {
        const row = this.shadowRoot.getElementById('skipListRow');
        if (!row) return;
        row.innerHTML = '';

        nodes.forEach((node, index) => {
            const div = document.createElement('div');
            div.className = 'px-4 py-2 rounded-lg bg-purple-100 border border-purple-400 text-purple-900 shadow-sm';
            div.innerText = `${node.value}`;
            row.appendChild(div);

            if (index < nodes.length - 1) {
                const arrow = document.createElement('span');
                arrow.className = 'text-gray-400 font-bold';
                arrow.innerText = '→';
                row.appendChild(arrow);
            }
        });
    }
}

customElements.define('skiplist-expiry-visual', SkipListExpiryVisual);