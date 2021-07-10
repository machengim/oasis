import { BaseSyntheticEvent, useState, useEffect } from 'react';
import Button from './Button';
import Spinner from './Spinner';
import * as api from '../utils/api';

interface IDirBrowserProps {
  onClose: () => void;
  onSelect: Function;
}

export default function DirBrowser(props: IDirBrowserProps) {
  const [currentDir, setCurrentDir] = useState('');
  const [dirs, setDirs] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [level, setLevel] = useState(0);
  const [back, setBack] = useState(false);
  const [selectedDir, setSelectedDir] = useState('');
  const [selectedVolume, setSelectedVolume] = useState('');
  const [volumes, setVolumes] = useState<string[]>([]);

  useEffect(() => {
    const fetchVolumes = async () => {
      try {
        const volumes: string[] = await api.get('/api/sys/volumes');
        setVolumes(volumes);
      } catch (e) {
        console.log(e);
        alert('Cannot get the volume list.');
      }
    };

    fetchVolumes();
  }, []);

  useEffect(() => {
    if (volumes.length > 0) {
      setSelectedVolume(volumes[0]);
    }
  }, [volumes]);

  useEffect(() => {
    if (selectedVolume) {
      setLevel(0);
      setSelectedDir(selectedVolume);
    }
  }, [selectedVolume]);

  useEffect(() => {
    if (selectedDir) {
      fetchDirs(selectedDir);
    }
  }, [selectedDir]);

  useEffect(() => {
    if (currentDir) {
      console.log('current dir: ', currentDir);
    }
  }, [currentDir]);

  const fetchDirs = async (dir: string) => {
    setIsLoading(true);

    try {
      const dirs: string[] = await api.get('/api/sys/dirs/' + dir);
      const newLevel = back ? level - 1 : level + 1;
      setCurrentDir(selectedDir);
      setDirs(dirs);
      setLevel(newLevel);
      setBack(false);
    } catch (e) {
      console.log(e);
      alert('Cannot read the directory.');
    } finally {
      setIsLoading(false);
    }
  };

  const onConfirmDir = () => {
    props.onSelect(currentDir);
    props.onClose();
  };

  // TODO: this works, but not ideal.
  const goToParentDir = (): void => {
    const dirSplit = currentDir.split('/').filter((e) => e.length > 0);
    if (currentDir.startsWith('/') && dirSplit.length > 0) {
      dirSplit[0] = '/' + dirSplit[0];
    }

    dirSplit.pop();
    const parentDir = dirSplit.length > 0 && dirSplit[0] ? dirSplit.join('/') : '/';
    setBack(true);
    setSelectedDir(parentDir);
  };

  return (
    <div className="fixed z-10 left-0 top-0 w-full h-full bg-black bg-opacity-40">
      <div className="bg-white w-96 my-40 mx-auto p-4 border rounded-lg border-gray-50 flex flex-col">
        <div className="mb-4 text-xl mx-auto text-gray-700">Directory Browser</div>
        <VolumeSelector volumes={volumes} setSelectedVolume={setSelectedVolume} />
        <hr />
        <div className="mt-2">Selected Directory:</div>
        <div className="text-gray-700 font-bold break-words mb-2">{currentDir}</div>
        {isLoading ? (
          <Spinner />
        ) : (
          <DirectoryList dirs={dirs} setSelectedDir={setSelectedDir} level={level} goToParentDir={goToParentDir} />
        )}
        <div className="mx-auto my-4 flex flex-row">
          <Button value="Confirm" className="mr-4" disabled={!currentDir} onClick={onConfirmDir} />
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
  setSelectedDir: (v: string) => void;
  goToParentDir: () => void;
  level: number;
}

function DirectoryList(props: IDirectoryListProps) {
  const formatDir = (dir: string): string => {
    const dirSplit = dir.split('/');
    return dirSplit[dirSplit.length - 1];
  };

  return (
    <div className="mb-2 py-2 border rounded h-60 overflow-y-auto overflow-x-hidden">
      {props.level > 1 && (
        <div className="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words" onClick={props.goToParentDir}>
          ..
        </div>
      )}
      {props.dirs.map((dir) => (
        <div
          key={dir}
          className="mx-2 px-2 rounded hover:bg-gray-200 cursor-pointer break-words"
          onClick={() => props.setSelectedDir(dir)}
        >
          {formatDir(dir)}
        </div>
      ))}
    </div>
  );
}
