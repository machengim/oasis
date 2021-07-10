import React, { BaseSyntheticEvent, useState, useEffect } from 'react';
import Button from '../components/Button';
import DirBrowser from '../components/DirBrowser';
import Notification from '../components/Notification';

export default function ServerInfoSetupPanel() {
  const [isOpenDirBrowser, setIsOpenDirBrowser] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isNoStorageError, setIsNoStorageError] = useState(false);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [selectedDir, setSelectedDir] = useState('');

  useEffect(() => {
    if (selectedDir) {
      setIsNoStorageError(false);
    }
  }, [selectedDir]);

  const onConfirm = (e: BaseSyntheticEvent) => {
    e.preventDefault();

    setIsSaving(true);
    if (!selectedDir || selectedDir.trim().length === 0) {
      setIsNoStorageError(true);
      return;
    }
    alert('submitted');
    setIsSaving(false);
  };

  const onChangeUsername = (e: BaseSyntheticEvent) => {
    setUsername(e.target.value);
  };

  const onChangePassword = (e: BaseSyntheticEvent) => {
    setPassword(e.target.value);
  };

  return (
    <div className="absolute w-full">
      <Notification type="success" />
      <form onSubmit={onConfirm}>
        <div className="w-96 mx-auto mt-40 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8 overflow-hidden">
          <div className="text-xl font-bold mb-8 text-gray-700">Server Setup</div>
          <div className="w-full grid grid-cols-4 mb-4">
            <div>username: </div>
            <div className="col-span-3">
              <input
                required
                minLength={1}
                maxLength={16}
                className="ml-2 w-40 border rounded focus:outline-none px-2"
                value={username}
                onChange={onChangeUsername}
              />
            </div>
          </div>
          <div className="w-full grid grid-cols-4 mb-4">
            <div>password: </div>
            <div className="col-span-3">
              <input
                required
                type="password"
                minLength={6}
                maxLength={16}
                className="ml-2 w-40 border rounded focus:outline-none px-2"
                value={password}
                onChange={onChangePassword}
              />
            </div>
          </div>
          <div className="w-full grid grid-cols-4 mb-12">
            <div>storage: </div>
            <div className="col-span-3 pl-2">
              {selectedDir ? (
                <>
                  <Button value={'Change'} onClick={() => setIsOpenDirBrowser(true)} />
                  <div className="mt-2 break-words">{selectedDir}</div>
                </>
              ) : (
                <Button value={'Select'} onClick={() => setIsOpenDirBrowser(true)} />
              )}
              {isNoStorageError && <div className="text-red-500">Please choose a storage</div>}
            </div>
          </div>
          <div className="mb-2">
            <Button value={'launch'} type={'important'} isSubmit={true} disabled={isSaving} />
          </div>
        </div>
        {isOpenDirBrowser && <DirBrowser onClose={() => setIsOpenDirBrowser(false)} onSelect={setSelectedDir} />}
      </form>
    </div>
  );
}
