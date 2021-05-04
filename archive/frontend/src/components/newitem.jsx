import React from 'react';

export default function NewItem() {
    return (
        <div id='new-item'>
            <TitleBar />
        </div>
    );

}

function TitleBar() {
    return (
        <div className='header-container'>
            <div className='title-bar'>
                NEW ITEMS
            </div>
        </div>
    )
}

function NewItemList() {
    return (
        <div className='new-item-list'>
            
        </div>
    )
}