import React, {useContext} from 'react';
import {LangContext} from '../../contexts/language';
import SubTitle from './subtitle';

export default function Step1(props) {
    const langContext = useContext(LangContext);

    function changeLang(e) {
        let newLang = e.target.value;
        props.setLanguage(newLang);
        langContext.setLanguage(newLang);
    }

    function changeUsername(e) {
        props.setUsername(e.target.value);
    }

    function changePassword(e) {
        props.setPassword(e.target.value);
    }

    function toggleAllowGuest() {
        props.setAllowGuest(!props.allowGuest);
    }

    return (
        <div className='text-2xl mt-8 flex flex-col space-y-6'>
            <SubTitle title="Basic Settings" />
            <table id='new-user-groups' className='text-lg bg-gray-100 text-left'>
                <tbody>
                <tr className='bg-green-400 text-white'>
                    <th>Option</th>
                    <th>Value</th>
                </tr>
                <tr>
                    <td>Language</td>
                    <td>
                        <select defaultValue={props.language} onChange={changeLang}
                            className='px-2 py-0.5'>
                            <option value='en'>English</option>
                            <option value='cn'>中文</option>
                        </select>
                    </td>
                </tr>
                <tr>
                    <td>
                        Admin username
                    </td>
                    <td>
                        <input type='text' defaultValue={props.username}
                            onChange={changeUsername}
                            className='w-48 px-2'
                        />
                    </td>
                </tr>
                <tr>
                    <td>
                        Admin password
                    </td>
                    <td>
                        <input type='password' defaultValue={props.password}
                            onChange={changePassword}
                            className='w-48 px-2'
                            
                        />
                    </td>
                </tr>
                <tr>
                    <td>Allow visits without login?</td>
                    <td>
                        <input type='checkbox' checked={props.allowGuest}
                            onChange={toggleAllowGuest} 
                        />
                        <span className='ml-2'>Yes</span>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
    );
}