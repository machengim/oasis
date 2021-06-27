import React from "react";
import Button from "../components/Button";

export default function ServerInfoSetupPanel() {
  return (
    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 ">
      <div className="w-96 bg-gray-50 shadow rounded-lg flex flex-col items-center px-8 py-4 overflow-hidden">
        <div className="text-lg mb-4">Server Setup</div>
        <div className="grid grid-cols-4 mb-4">
          <div className="col-span-1">username: </div>
          <div>
            <input className="ml-2 w-40 border rounded focus:outline-none px-2" />
          </div>
        </div>
        <div className="grid grid-cols-4 mb-4">
          <div className="col-span-1">password: </div>
          <div>
            <input type="password" className="ml-2 w-40 border rounded focus:outline-none px-2" />
          </div>
        </div>
        <div className="grid grid-cols-4 mb-12">
          <div className="col-span-1">storage: </div>
          <div>
            <input type="file" className="ml-2" />
          </div>
        </div>
        <div className="mb-2">
          <Button value={"launch"} />
        </div>
      </div>
    </div>
  );
}
