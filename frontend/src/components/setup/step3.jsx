import React from 'react';
import SubTitle from './subtitle';

export default function Step3() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <SubTitle title="Categories" />
            <table id='new-user-groups' className='text-lg bg-gray-100 text-left'>
                <tbody>
                <tr className='bg-green-400 text-white'>
                    <th>Category</th>
                    <th>Priviledge required</th>
                    <th>Order</th>
                </tr>
                <tr>
                    <td>
                        Movie
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
                        1
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                </tr>
                <tr>
                    <td>
                        TV series
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
                        2
                        <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                            <ion-icon name="create-outline"></ion-icon>
                        </div> 
                    </td>
                </tr>
                </tbody>
            </table>
            <div className='pt-4 text-lg'>
                <input type='button' value='Add category' 
                    className='bg-green-400 py-0.5 px-2 text-white rounded cursor-pointer'
                />
            </div>
        </div>
    );
}