import * as preact from 'preact';
import * as hooks from 'preact/hooks'
import * as wasm from './pkg/glue'

function hex(n: number) {
  return n.toString(16).padStart(8, '0');
}

type EffectsMode = 'instr' | 'block';

function Dis(props: { ana: wasm.Analysis, effMode: EffectsMode }) {
  const { ana, effMode } = props;

  const blocks = ana.blocks.map(({ start, end, effects }) => {
    const rows = [];
    for (let i = start; i < end; i++) {
      const inst = ana.instrs[i];
      let eff;
      if (effMode === 'instr') {
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

    let eff;
    if (effMode === 'block') {
      eff = <div class='effect' >{effects.map(e => <div>{e}</div>)}</div>;
    }

    return <div class='block'><table>{rows}</table>{eff}</div>;
  });
  return <div class='dis'>{blocks}</div>;
}

function Body(props: { ana: wasm.Analysis }) {
  const [effects, setEffects] = hooks.useState<EffectsMode>('instr');

  const setSelectValue = (e: Event) => {
    setEffects((e.currentTarget! as HTMLInputElement).value as EffectsMode);
  };

  return <main>
    <div style={{ padding: '1em' }}>
      effects:{' '}
      <select name='effects' onChange={setSelectValue}>
        <option value='instr'>per instruction</option>
        <option value='block'>per block</option>
      </select>
    </div>
    <Dis ana={props.ana} effMode={effects} />
  </main>;
}

export default async function () {
  await wasm.default();
  const ana = wasm.sample();
  preact.render(<Body ana={ana} />, document.body);
}
