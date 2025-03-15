import * as preact from 'preact';
import * as wasm from './pkg/glue'

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

function dis(ana: wasm.Analysis) {
  const beff = false;
  const blocks = ana.blocks.map(({ start, end, effects }) => {
    const rows = [];
    for (let i = start; i < end; i++) {
      const inst = ana.instrs[i];
      let eff;
      if (beff) {
        if (i == start) {
          eff = <td class='effect' rowspan={end - start}>{effects.map(e => <div>{e}</div>)}</td>;
        }
      } else {
        eff = <td class='effect'>{inst.effects.map(e => <div>{e}</div>)}</td>;
      }
      rows.push(
        <tr>
          <td class='addr'>{hex(inst.ip)}</td>
          <td class='asm'>{inst.asm}</td>
          {eff}
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
