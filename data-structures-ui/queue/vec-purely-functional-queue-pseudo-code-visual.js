class VecPurelyFunctionalQueuePseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.load();
    }

    async load() {
        const [html, css] = await Promise.all([
            fetch('./queue/vec-purely-functional-queue-pseudo-code-visual.html').then(r => r.text()),
            fetch('./queue/vec-purely-functional-queue-pseudo-code-visual.css').then(r => r.text()),
        ]);

        this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;
    }
}

customElements.define(
    'purely-functional-queue-vec-pseudo-code-visual-viewer',
    VecPurelyFunctionalQueuePseudoCodeVisual
);