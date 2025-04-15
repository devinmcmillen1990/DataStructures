class QueueCommandRibbon extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.ready = this.loadTemplate();
  }

  async loadTemplate() {
    const html = await fetch('./queue/queue-command-ribbon.html').then(res => res.text());
    this.shadowRoot.innerHTML = html;

    const enqueueBtn = this.shadowRoot.getElementById('enqueueBtn');
    const dequeueBtn = this.shadowRoot.getElementById('dequeueBtn');
    const clearBtn = this.shadowRoot.getElementById('clearQueueBtn');
    const input = this.shadowRoot.getElementById('enqueueInput');

    enqueueBtn?.addEventListener('click', () => {
      const value = input.value.trim();
      if (value !== '') {
        this.dispatchEvent(new CustomEvent('enqueue', { detail: value, bubbles: true }));
        input.value = '';
      }
    });

    dequeueBtn?.addEventListener('click', () => {
      this.dispatchEvent(new Event('dequeue', { bubbles: true }));
    });

    clearBtn?.addEventListener('click', () => {
      this.dispatchEvent(new Event('clear-queue', { bubbles: true }));
    });
  }
}

customElements.define('queue-command-ribbon', QueueCommandRibbon);
