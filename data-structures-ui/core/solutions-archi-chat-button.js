class SiteHeader extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    async connectedCallback() {
        const html = await fetch('./core/solutions-archi-chat-button.html').then(res => res.text());
        this.shadowRoot.innerHTML = html;
    }
}

customElements.define('solutions-archi-chat-button', SiteHeader);
