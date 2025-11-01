import { OptionBlock as OptionBlockType } from '@/lib/types';

interface OptionBlockProps {
  block: OptionBlockType;
}

export const OptionBlock = ({ block }: OptionBlockProps) => (
  <div className="my-2">
    <h3>{block.name}</h3>
    <ul className="text-justify my-3 ml-5">
      {block.values.map((opt, i) => (
        <li className="list-disc" key={i}>
          {opt}
        </li>
      ))}
    </ul>
  </div>
);
