class ListQueueVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/list-purely-functional-queue-visual.html').then(res => res.text()),
            fetch('./index.css').then(res => res.text())
        ]);

        const style = document.createElement('style');
        style.textContent = css;

        const container = document.createElement('div');
        container.innerHTML = html;

        this.shadowRoot.innerHTML = '';
        this.shadowRoot.appendChild(style);
        this.shadowRoot.appendChild(container);
    }

    update(state) {
        const frontRow = this.shadowRoot.getElementById('frontRow');
        const rearRow = this.shadowRoot.getElementById('rearRow');
        if (!frontRow || !rearRow) return;

        const renderLinkedRow = (container, list) => {
            container.innerHTML = '';

            if (list.length === 0) {
                const placeholder = document.createElement('div');
                placeholder.className = 'w-10 h-10 opacity-0';
                container.appendChild(placeholder);
                return;
            }

            list.forEach((val, idx) => {
                const node = document.createElement('div');
                node.className =
                    'px-4 py-2 bg-indigo-100 text-indigo-800 font-mono text-sm rounded-full border border-indigo-300 shadow-sm whitespace-nowrap';
                node.textContent = val;
                container.appendChild(node);

                if (idx < list.length - 1) {
                    const arrow = document.createElement('div');
                    arrow.className = 'text-gray-600 font-bold text-xl';
                    arrow.textContent = '→';
                    container.appendChild(arrow);
                }
            });
        };

        renderLinkedRow(frontRow, state.front);
        renderLinkedRow(rearRow, [...state.rear].reverse());
    }
}

customElements.define('purely-functional-queue-list-visual', ListQueueVisual);  