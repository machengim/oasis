import React from 'react';
import SubTitle from './subtitle';

export default function Step2(props) {
    const groups = props.groups;
    const setGroups = props.setGroups;

    function toggleDefault(name) {
        let newGroup = groups.slice();
        let target = newGroup.filter((g) => g.name === name)[0];
        if (!target) return;

        target.default = !target.default;
        if (target.default) {
            newGroup.filter((g) => g.name !== name).map((g) => g.default = false);
        }
        setGroups(newGroup);
    }

    function changePower(name) {
        let newPower = window.prompt("Enter a new power between 1 and 9");
        if (newPower === null || newPower.trim().length === 0) return;
        if (isNaN(newPower) || newPower < 1 || newPower > 9) {
            alert("Invalid input. Please try again.");
            return;
        }

        let newGroup = groups.slice();
        let target = newGroup.filter((g) => g.name === name)[0];
        if (!target) return;
        target.power = newPower;
        setGroups(newGroup);
    }

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
                {groups.map((group) => <tr key={group.name}>
                    <td>{group.name}
                        {group.delete &&                         
                            <div className='hidden-btn'  onClick={() => alert('Clicked')}>
                                <ion-icon name="trash-outline"></ion-icon>
                            </div> }
                    </td>
                    <td>{group.power}
                        {group.editPower &&  
                            <div className='hidden-btn'  onClick={() => changePower(group.name)}>
                                <ion-icon name="create-outline"></ion-icon>
                            </div> }
                    </td>
                    <td>{group.editDefault && 
                        <input type='checkbox' 
                            checked={group.default}
                            onChange={() => toggleDefault(group.name)}
                        />}
                        {!group.editDefault && <div>-</div>}
                    </td>
                </tr>)}

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