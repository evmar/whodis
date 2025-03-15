import * as preact from 'preact';

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

export default function (instrs: JSInst[]) {
  preact.render(dis(instrs), document.body);
}
