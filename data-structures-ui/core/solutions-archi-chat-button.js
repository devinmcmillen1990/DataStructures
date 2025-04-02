class SiteHeader extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    async connectedCallback() {
        const [html, css] = await Promise.all([
            fetch('./core/solutions-archi-chat-button.html').then(res => res.text()),
            fetch('./core/solutions-archi-chat-button.css').then(res => res.text())
        ]);

        this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;
    }
}

customElements.define('solutions-archi-chat-button', SiteHeader);
