import React from 'react';
import SubTitle from './subtitle';

export default function Step4() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <SubTitle title="Storagies" />
            <table id='new-user-groups' className='text-lg bg-gray-100 text-left'>
                <tbody>
                <tr className='bg-green-400 text-white'>
                    <th>Name</th>
                    <th>Path</th>
                    <th>Default category</th>
                    <th>Auto scan</th>
                </tr>
                <tr>
                    <td>
                        Shows
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="trash-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        D:/Myshows
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        Movie
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        <input type='checkbox'/>
                    </td>
                </tr>
                <tr>
                    <td>
                        Photos
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="trash-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        E:/Images/FamilyPhoto
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                    <td>
                        -
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
                <input type='button' value='Add storage' 
                    className='bg-green-400 py-0.5 px-2 text-white rounded cursor-pointer'
                />
            </div>
        </div>
    );
}