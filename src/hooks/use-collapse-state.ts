import { all, fromPairs } from "ramda";
import { useMemo, useState } from "react";

export const useCollapseState = <T extends (string | number)>(
  ids: T[],
) => {
  const [collapseState, setCollapsedState] = useState<Record<T, boolean>>({} as Record<T, boolean>);

  const toggleCollapse = (id: T) => {
    setCollapsedState({
      ...collapseState,
      [id]: !collapseState[id],
    });
  };

  const allCollapsed = useMemo(() => {
    if (ids.length === 0) {
      return false;
    }
    return all(id => !!collapseState[id], ids)
  }, [collapseState, ids]);

  const toggleAll = () => {
    const toggleState = !allCollapsed;
    setCollapsedState(fromPairs(ids.map(id => [id, toggleState])));
  }

  const isCollapsed = (id: T) => collapseState[id];

  return {
    toggleCollapse,
    toggleAll,
    isCollapsed,
    allCollapsed,
  };
};