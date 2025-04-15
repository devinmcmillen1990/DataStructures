class VecPurelyFunctionalQueuePseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.load();
    }

    async load() {
        const html = await fetch('./queue/vec-purely-functional-queue-pseudo-code-visual.html').then(r => r.text());
        this.shadowRoot.innerHTML = html;
    }
}

customElements.define(
    'purely-functional-queue-vec-pseudo-code-visual-viewer',
    VecPurelyFunctionalQueuePseudoCodeVisual
);
