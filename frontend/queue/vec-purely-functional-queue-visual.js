class QueueStateVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/vec-purely-functional-queue-visual.html').then(res => res.text()),
            fetch('./queue/vec-purely-functional-queue-visual.css').then(res => res.text())
        ]);

        this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;
    }

    update(state) {
        const frontRow = this.shadowRoot.getElementById('frontRow');
        const rearRow = this.shadowRoot.getElementById('rearRow');
        if (!frontRow || !rearRow) return;

        frontRow.innerHTML = '';
        rearRow.innerHTML = '';

        state.front.forEach(val => {
            const box = document.createElement('div');
            box.className = 'queue-box';
            box.textContent = val;
            frontRow.appendChild(box);
        });

        [...state.rear].reverse().forEach(val => {
            const box = document.createElement('div');
            box.className = 'queue-box';
            box.textContent = val;
            rearRow.appendChild(box);
        });
    }
}

customElements.define('vec-purely-functional-queue-visual', QueueStateVisual);
