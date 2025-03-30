class VecPurelyFunctionalQueuePseudoCodeVisual extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.ready = this.loadTemplate();
    }

    async loadTemplate() {
        const [html, css] = await Promise.all([
            fetch('./queue/vec-purely-functional-queue-pseudo-code-visual.html').then(res => res.text()),
            fetch('./queue/vec-purely-functional-queue-pseudo-code-visual.css').then(res => res.text())
        ]);

        this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;
        this.codeBlock = this.shadowRoot.getElementById('codeBlock');
    }

    setOperation(op, value = null) {
        if (!this.codeBlock) return;

        const code = {
            initial: `signature QUEUE =
  sig
    type 'a Queue
    exception EMPTY
    val empty   : 'a Queue
    val isEmpty : 'a Queue -> bool
    val snoc    : 'a Queue -> 'a -> 'a Queue
    val head    : 'a Queue -> 'a        (* raises EMPTY if queue is empty *)
    val tail    : 'a Queue -> 'a Queue  (* raises EMPTY if queue is empty *)
  end`,

            enqueue: `fun snoc (q, x) =
    let
      val rear' = x :: q.rear
    in
      if null q.front then
        { front = rev rear', rear = [] }
      else
        { front = q.front, rear = rear' }
    end`,

            dequeue: `fun tail q =
    case q.front of
      [] =>
        (case rev q.rear of
           [] => raise EMPTY
         | x::xs => { front = xs, rear = [] })
    | _::xs => { front = xs, rear = q.rear }`
        };

        if (op === 'enqueue') {
            this.codeBlock.textContent = code.enqueue;
        } else if (op === 'dequeue') {
            this.codeBlock.textContent = code.dequeue;
        } else {
            this.codeBlock.textContent = code.initial;
        }
    }
}

customElements.define('vec-purely-functional-queue-pseudo-code-visual-viewer', VecPurelyFunctionalQueuePseudoCodeVisual);