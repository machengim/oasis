import React, { useState } from 'react';
import Button from '../components/Button';
import DirBrowser from '../components/DirBrowser';

export default function ServerInfoSetupPanel() {
  const [isOpenDirBrowser, setIsOpenDirBrowser] = useState(false);

  return (
    <div className="absolute w-full">
      <div className="w-96 mx-auto mt-40 bg-gray-50 shadow rounded-lg flex flex-col items-center px-8 py-4 overflow-hidden">
        <div className="text-lg mb-4 text-gray-700">Server Setup</div>
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
            <Button value={'Select'} onClick={() => setIsOpenDirBrowser(true)} />
          </div>
        </div>
        <div className="mb-2">
          <Button value={'launch'} style={'important'} />
        </div>
      </div>
      {isOpenDirBrowser && <DirBrowser onClose={() => setIsOpenDirBrowser(false)} />}
    </div>
  );
}
