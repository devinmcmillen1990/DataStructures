// TODO: Get Paper or resources on this data structure.
class SkipListExpiryPseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./time-indexed/skiplist/skiplist-expiry-pseudo-code-visual.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;
    }
}

customElements.define('skiplist-expiry-pseudo-code-visual', SkipListExpiryPseudoCodeVisual);