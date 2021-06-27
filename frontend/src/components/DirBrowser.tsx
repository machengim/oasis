import React, { BaseSyntheticEvent, useState } from 'react';
import Button from './Button';

interface IDirBrowserProps {
  onClose: () => void;
}

export default function DirBrowser(props: IDirBrowserProps) {
  const [selectedVolume, setSelectedVolume] = useState('');
  const [selectedDirectory, setSelectedDirectory] = useState('');
  const volumes = ['C', 'Destiny', 'E'];
  const dirs = ['..', 'Documents', 'Downloads', 'Movies', 'Temporary salsafkssdssfdsfsafafdasfaslajdkfakfdkafdhskf'];

  return (
    <div className="fixed z-10 left-0 top-0 w-full h-full bg-black bg-opacity-40">
      <div className="bg-white w-96 my-40 mx-auto p-4 border rounded-lg border-gray-50 flex flex-col">
        <div className="mb-4 text-xl mx-auto text-gray-700">Directory Browser</div>
        <VolumeSelector volumes={volumes} setSelectedVolume={setSelectedVolume} />
        <hr />
        <DirectoryList dirs={dirs} setSelectedDirectory={setSelectedDirectory} />
        <hr />
        <div className="mt-2">Selected Directory:</div>
        <div className="mb-8 text-gray-700 break-words">{selectedDirectory}</div>
        <div className="mx-auto flex flex-row">
          <Button value="Confirm" className="mr-4" />
          <Button value="Cancel" onClick={props.onClose} />
        </div>
      </div>
    </div>
  );
}

interface IVolumeSelectorProps {
  volumes: string[];
  setSelectedVolume: (v: string) => void;
}

function VolumeSelector(props: IVolumeSelectorProps) {
  const selectVolume = (e: BaseSyntheticEvent) => {
    props.setSelectedVolume(e.target.value);
  };

  return (
    <div className="mb-4 flex flex-row items-center">
      <span className="mr-4">Volumes:</span>
      <select className="px-2 border bg-gray-50" onChange={selectVolume}>
        {props.volumes.map((volume) => (
          <option key={volume} value={volume}>
            {volume}
          </option>
        ))}
      </select>
    </div>
  );
}

interface IDirectoryListProps {
  dirs: string[];
  setSelectedDirectory: (v: string) => void;
}

function DirectoryList(props: IDirectoryListProps) {
  return (
    <div className="mb-2 h-60 overflow-y-auto overflow-x-hidden">
      {props.dirs.map((dir) => (
        <div
          key={dir}
          className="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
          onClick={() => props.setSelectedDirectory(dir)}
        >
          {dir}
        </div>
      ))}
    </div>
  );
}
