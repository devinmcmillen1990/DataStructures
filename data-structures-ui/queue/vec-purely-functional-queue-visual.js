class QueueStateVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./queue/vec-purely-functional-queue-visual.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;
    }

    update(state) {
        const frontRow = this.shadowRoot.getElementById('frontRow');
        const rearRow = this.shadowRoot.getElementById('rearRow');
        if (!frontRow || !rearRow) return;

        frontRow.innerHTML = '';
        rearRow.innerHTML = '';

        const renderBox = (val) => {
            const box = document.createElement('div');
            box.className = 'px-3 py-1 bg-blue-100 border border-blue-400 rounded text-sm';
            box.textContent = val;
            return box;
        };

        state.front.forEach(val => frontRow.appendChild(renderBox(val)));
        [...state.rear].reverse().forEach(val => rearRow.appendChild(renderBox(val)));
    }
}

customElements.define('purely-functional-queue-vec-visual', QueueStateVisual);
