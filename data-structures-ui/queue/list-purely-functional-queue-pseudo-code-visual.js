class ListPurelyFunctionalQueuePseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.load();
    }

    async load() {
        const html = await fetch('./queue/list-purely-functional-queue-pseudo-code-visual.html').then(r => r.text());
        this.shadowRoot.innerHTML = html;
    }
}

customElements.define(
    'purely-functional-queue-list-pseudo-code-visual-viewer',
    ListPurelyFunctionalQueuePseudoCodeVisual
);
