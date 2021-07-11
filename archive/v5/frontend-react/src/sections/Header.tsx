import { useContext } from "react";
import { AppContext } from "../context";

export default function Header() {
  const appContext = useContext(AppContext);
  const { sections } = appContext;

  const buildNav = () => {
    let nav = <span>Oasis</span>;
    if (sections.length > 0 && sections[0] === "/setup") {
      nav = (
        <span>
          <a href="/">Oasis</a> &gt; Setup
        </span>
      );
    }
    return nav;
  };

  return (
    <div className="w-full h-14 bg-gray-50 shadow">
      <div className="w-4/5 h-full flex flex-row justify-between items-center mx-auto">
        <div className="text-xl">{buildNav()}</div>
      </div>
    </div>
  );
}
