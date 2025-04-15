class ListPurelyFunctionalQueuePseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/purely-functional-queue-pseudo-code-visual.html').then(res => res.text()),
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
}

customElements.define(
    'purely-functional-queue-pseudo-code-visual',
    ListPurelyFunctionalQueuePseudoCodeVisual
);
