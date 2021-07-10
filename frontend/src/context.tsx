import { useState, createContext, ReactNode } from "react";

interface IAppContext {
  sections: string[];
  setSections: Function;
}

export const AppContext = createContext<IAppContext>({ sections: [], setSections: () => {} });

export function AppCtxProvider(props: { children: ReactNode }) {
  const [sections, setSections] = useState([]);

  return <AppContext.Provider value={{ sections, setSections }}>{props.children}</AppContext.Provider>;
}
