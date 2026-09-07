import { FullOptionBlock } from "@/types/option_blocks";

interface OptionBlockProps {
  hideTitle?: boolean;
  block: FullOptionBlock;
}

export const OptionBlock = ({ block, hideTitle = false }: OptionBlockProps) => (
  <div className="my-2">
    {!hideTitle && <h3>{block.name}</h3>}
    <ul className="text-justify my-3 ml-5">
      {block.entries.map((opt, i) => (
        <li className="list-disc" key={i}>
          {opt}
        </li>
      ))}
    </ul>
  </div>
);
