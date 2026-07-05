import { cn } from '@/lib/utils';
import { openUrl } from '@tauri-apps/plugin-opener';

interface ExternalLinkProps {
  children: React.ReactNode;
  href: string;
  className?: string;
}

// This is needed because anchor tags will open a URL in the tauri webview,
// which isn't great. This will open a URL in the user's browser.
export const ExtLink = ({ href, children, className }: ExternalLinkProps) => (
  <span
    className={cn(
      'font-bold underline cursor-pointer hover:text-(--color-chart-2)',
      className,
    )}
    onClick={() => {
      openUrl(href);
    }}
  >
    {children}
  </span>
);
