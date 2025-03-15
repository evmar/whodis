import * as preact from 'preact';
import * as wasm from './pkg/glue'

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

function dis(instrs: wasm.Instr[]) {
  // <td class='effect'>{inst.Effects?.map(e => <div>{e}</div>)}</td>
  const rows = instrs.map((inst) => (
    <tr>
      <td class='addr'>{hex(inst.ip)}</td>
      <td>{inst.asm}</td>
    </tr>
  ));
  return <table>{rows}</table>;
}

export default async function () {
  await wasm.default();
  const instrs = wasm.sample();
  preact.render(dis(instrs), document.body);
}
