class BfMemory extends HTMLElement {
  constructor() {
    super();

    let length = Number(this.attributes.getNamedItem('length').value);

    const table = document.createElement('table');
    table.style.margin = '3rem 0';

    const tbody = document.createElement('tbody');
    const tr = document.createElement('tr');

    const pointer = this.attributes.pointer ? Number(this.attributes.pointer.value) : 0;

    for (let i = 0; i < length; i++) {
      const td = document.createElement('td');

      const cell = this.attributes.getNamedItem(`c${i}`);
      td.innerText = cell ? cell.value : '0';

      if (pointer == i) {
        td.title = `Index = ${i} (Pointer is here)`;
        td.classList.add('pointer');
      } else {
        td.title = `Index = ${i}`;
      }

      tr.appendChild(td);
    }

    tbody.appendChild(tr);
    table.appendChild(tbody);

    this.appendChild(table);
  }
}

window.customElements.define('bf-memory', BfMemory);
