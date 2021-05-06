import React, {useContext, useState} from 'react';
import ReactDom from 'react-dom';
import LanguageProvider, {LangContext} from './contexts/language';
import { useTranslation } from 'react-i18next';
import './i18n/config';

ReactDom.render(
    <App />, 
    document.getElementById('root')
);

function App() {
    const [step, setStep] = useState(4);

	return (
        <LanguageProvider>
            <div className='w-screen h-screen'>
                <div className='w-3/5 mx-auto text-center'>
                    <Title />
                    {step == 1 && <Step1 />}
                    {step == 2 && <Step2 />}
                    {step == 3 && <Step3 />}
                    {step == 4 && <Step4 />}
                    <FootButtons step={step} setStep={setStep} />
                </div>
            </div>
        </LanguageProvider>
    );
}

function Title() {
    const { t } = useTranslation();

    return (
        <div className='text-center text-3xl font-bold mt-16'>
            {t('OASIS Setup')}
        </div>
    );
}

function Step1() {
    const langContext = useContext(LangContext);
    const { t } = useTranslation();

    function changeLang(e) {
        langContext.setLanguage(e.target.value);
    }

    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div>{t('Please choose a language')} :</div>
            <div className='w-full'>
                <select id='lang' defaultValue='en' 
                    className='w-32 text-xl mx-auto'
                    onChange={changeLang}>
                    <option value='en'>English</option>
                    <option value='cn'>中文</option>
                </select>
            </div>

        </div>
    )
}

function Step2() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                Admin info
            </div>
            <div className='flex space-x-8 justify-center'>
                <span>Username: </span>
                <input type='input' className='border-2' required/>
            </div>
            <div className='flex space-x-8 justify-center'>
                <span>Password: </span>
                <input type='password' className='border-2' required/>
            </div>
        </div>
    )
}

function Step3() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                User groups
            </div>
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

function Step4() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                Categories
            </div>
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

function FootButtons(props) {
    const step = props.step;
    const setStep = props.setStep;

    function moveStep(i) {
        setStep(step + i);
    }

    return (
        <div className='relative mt-16'>
            {step !== 1 && 
                <input type='button' value='<< PREV' 
                    className='absolute text-xl left-4 top-0 cursor-pointer px-4 py-1 rounded bg-blue-400 text-white'
                    onClick={() => moveStep(-1)} 
                />
            }
            <input type='button' value='NEXT >>' 
                className='absolute text-xl right-4 top-0 cursor-pointer px-4 py-1 rounded bg-blue-400 text-white'
                onClick={() => moveStep(1)} 
            />
        </div>
    )
}