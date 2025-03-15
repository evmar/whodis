import * as preact from 'preact';
import * as wasm from './pkg/glue'

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

function dis(ana: wasm.Analysis) {
  const blocks = ana.blocks.map(({ start, end }) => {
    const rows = [];
    for (let i = start; i < end; i++) {
      const inst = ana.instrs[i];
      rows.push(
        <tr>
          <td class='addr'>{hex(inst.ip)}</td>
          <td class='asm'>{inst.asm}</td>
          <td class='effect'>{inst.effects.map(e => <div>{e}</div>)}</td>
        </tr>
      );
    }
    return <table class='block'>{rows}</table>;
  });
  return <div>{blocks}</div>;
}

export default async function () {
  await wasm.default();
  const ana = wasm.sample();
  preact.render(dis(ana), document.body);
}
