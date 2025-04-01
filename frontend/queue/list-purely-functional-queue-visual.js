class ListQueueVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/list-purely-functional-queue-visual.html').then(res => res.text()),
            fetch('./queue/list-purely-functional-queue-visual.css').then(res => res.text())
        ]);

        this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;
    }

    update(state) {
        const frontRow = this.shadowRoot.getElementById('frontRow');
        const rearRow = this.shadowRoot.getElementById('rearRow');
        if (!frontRow || !rearRow) return;

        const renderLinkedRow = (container, list) => {
            container.innerHTML = '';
            list.forEach((val, idx) => {
                const node = document.createElement('div');
                node.className = 'node';
                node.textContent = val;
                container.appendChild(node);

                if (idx < list.length - 1) {
                    const arrow = document.createElement('span');
                    arrow.className = 'arrow';
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
