import * as preact from 'preact';
import * as wasm from './pkg/glue'

type JSInst = any;

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

function dis(instrs: JSInst[]) {
  const rows = instrs.map((inst) => (
    <tr>
      <td class='addr'>{hex(inst.Eip)}</td>
      <td>{inst.Op}</td>
      <td class='effect'>{inst.Effects?.map(e => <div>{e}</div>)}</td>
    </tr>
  ));
  return <table>{rows}</table>;
}

export default async function (instrs: JSInst[]) {
  await wasm.default();
  wasm.greet("hello");
  preact.render(dis(instrs), document.body);
}
