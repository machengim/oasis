import React from 'react';
import SubTitle from './subtitle';

export default function Step4() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <SubTitle title="Congrats" />
            <div className='text-center'>
                <p>Configuration is done.</p> 
                <p>Finilize it and launch your site:</p>
            </div>
            <div>
                <input type='button' value='LAUNCH'
                    className='mt-4 bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-500 transit-300' 
                />
            </div>
        </div>
    );
}