import { Link } from "react-router-dom";

export default function Home() {
  return (
    <div className="text-md">
      {" "}
      <nav>
        <ul>
          <li>
            <Link to="/about">About</Link>
          </li>
          <li>
            <Link to="/users">Users</Link>
          </li>
        </ul>
      </nav>
    </div>
  );
}
