import { Paragraph } from "@/components/ui/paragraph";
import { cn } from "@/lib/utils";

export const Grid = ({ children }: { children: React.ReactNode }) => (
  <div className={cn('mt-8 masonry')}>
    {children}
  </div>
);

export const SpecialInfo = ({ text }: { text: string; }) => {
  const [first, ...rest] = text.split('|').map(s => s.trim());

  switch (first.toLowerCase()) {
    case 'condition': {
      return (
        <p className="text-justify my-3"><b>Condition:</b> {rest[0]}</p>
      );
    }
    case 'conditions': {
      return (
        <>
          <p className="text-justify"><b>Conditions:</b></p>
          <ul className="text-justify my-3 ml-5">
            {rest.map((cond, i) => (
              <li className="list-disc" key={i}>{cond}</li>
            ))}
          </ul>
        </>
      );
    }
    default: {
      return (
        <Paragraph>{first}</Paragraph>
      );
    }
  }
};
