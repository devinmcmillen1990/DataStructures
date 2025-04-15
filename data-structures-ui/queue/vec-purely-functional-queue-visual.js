class QueueStateVisual extends HTMLElement {
    constructor() {
        super();
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const html = await fetch('./queue/vec-purely-functional-queue-visual.html').then(res => res.text());
        this.innerHTML = html;
    }

    update(state) {
        const frontRow = this.querySelector('#frontRow');
        const rearRow = this.querySelector('#rearRow');
        if (!frontRow || !rearRow) return;

        frontRow.innerHTML = '';
        rearRow.innerHTML = '';

        const renderBox = (val) => {
            const box = document.createElement('div');

            box.classList.add(
                'h-10',
                'px-3',
                'flex', 'items-center', 'justify-center',
                'bg-blue-100', 'border', 'border-blue-500',
                'rounded', 'text-sm', 'font-mono',
                'shadow-sm',
                'whitespace-nowrap'
            );

            box.textContent = val;
            return box;
        };

        const renderPlaceholder = () => {
            const box = document.createElement('div');
            box.classList.add('w-10', 'h-10', 'opacity-0');
            return box;
        };

        if (state.front.length === 0) {
            frontRow.appendChild(renderPlaceholder());
        } else {
            state.front.forEach(val => frontRow.appendChild(renderBox(val)));
        }

        if (state.rear.length === 0) {
            rearRow.appendChild(renderPlaceholder());
        } else {
            [...state.rear].reverse().forEach(val => rearRow.appendChild(renderBox(val)));
        }
    }

}

customElements.define('purely-functional-queue-vec-visual', QueueStateVisual);
