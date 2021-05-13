import React from 'react';
import SubTitle from './subtitle';

export default function Step2() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <SubTitle title="User Groups" />
            <table id='new-user-groups' className='text-lg bg-gray-100 text-left'>
                <tbody>
                <tr className='bg-green-400 text-white'>
                    <th>Group name</th>
                    <th>Priviledge</th>
                    <th>Default group</th>
                </tr>
                <tr>
                    <td>Admin</td>
                    <td>9</td>
                    <td>-</td>
                </tr>
                <tr>
                    <td>User</td>
                    <td>3</td>
                    <td>
                        <input type='checkbox' defaultChecked />
                    </td>
                </tr>
                <tr>
                    <td>
                        Kid
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="trash-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        1
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        <input type='checkbox' />
                    </td>
                </tr>
                </tbody>
            </table>
            <div className='pt-4 text-lg'>
                <input type='button' value='Add group' 
                    className='bg-green-400 py-0.5 px-2 text-white rounded cursor-pointer'
                />
            </div>
        </div>
    );
}