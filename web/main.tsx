import * as preact from 'preact';
import * as hooks from 'preact/hooks'
import * as wasm from './pkg/glue'

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

function Dis(props: { program: wasm.Program, count: number }) {
  const { program, count } = props;
  const ana = wasm.sample(props.program, count);

  const blocks = ana.blocks.map(({ start, end }) => {
    const rows = [];
    for (let i = start; i < end; i++) {
      const inst = ana.instrs[i];
      const eff = <td class='effect'>{inst.effects.map(e => <div>{e}</div>)}</td>;
      rows.push(
        <tr>
          <td class='addr'>{hex(inst.ip)}</td>
          <td class='asm'>{inst.lit ? '* ' : ''}{inst.asm}</td>
          {eff}
        </tr>
      );
    }

    return <div class='block'><table>{rows}</table></div>;
  });
  const eff = <div class='effect'>{ana.effects.map(e => <div>{e}</div>)}</div>;
  return <div class='dis'><div class='blocks'>{blocks}</div>{eff}</div>;
}

function Body(props: { program: wasm.Program }) {
  const [count, setCount] = hooks.useState(0);

  return <main>
    <input type='range' min='0' max='20' value={count} onInput={e => setCount(parseInt(e.currentTarget.value))} />
    <div>
      <Dis program={props.program} count={count} />
    </div>
  </main>;
}

export default async function () {
  await wasm.default();
  const program = wasm.init();

  preact.render(<Body program={program} />, document.body);
}
