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
    const [step, setStep] = useState(1);

	return (
        <LanguageProvider>
            <div className='w-full h-full bg-white'>
                <div className='w-4/5 lg:w-1/2 mx-auto text-center'>
                    <Title />
                    {step == 1 && <Step1 />}
                    {step == 2 && <Step2 />}
                    {step == 3 && <Step3 />}
                    {step == 4 && <Step4 />}
                    {step == 5 && <Step5 />}
                    <FootButtons step={step} setStep={setStep} />
                </div>
            </div>
        </LanguageProvider>
    );
}

function Title() {
    const { t } = useTranslation();

    return (
        <div className='text-center text-3xl font-bold mt-12'>
            {t('OASIS Setup')}
        </div>
    );
}

function Step1() {
    const langContext = useContext(LangContext);

    function changeLang(e) {
        langContext.setLanguage(e.target.value);
    }

    return (
        <div className='text-2xl mt-8 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                Basic settings
            </div>
            <table id='new-user-groups' className='text-lg bg-gray-100 text-left'>
                <tbody>
                <tr className='bg-green-400 text-white'>
                    <th>Option</th>
                    <th>Value</th>
                </tr>
                <tr>
                    <td>Language</td>
                    <td>
                        <select defaultValue='en' onChange={changeLang}
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
                        <input type='text' defaultValue='admin' 
                            className='w-48 px-2'
                            
                        />
                    </td>
                </tr>
                <tr>
                    <td>
                        Admin password
                    </td>
                    <td>
                        <input type='password' 
                            className='w-48 px-2'
                            
                        />
                    </td>
                </tr>
                <tr>
                    <td>Allow visits without login?</td>
                    <td>
                        <input type='checkbox' defaultChecked />
                        <span className='ml-2'>Yes</span>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
    )
}

function Step2() {
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

function Step3() {
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

function Step4() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                Storages
            </div>
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

function Step5() {
    return (
        <div className='text-2xl mt-12 flex flex-col space-y-6'>
            <div className='font-semibold pb-4'>
                Congrats
            </div>
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
                    className='foot-btn left-4'
                    onClick={() => moveStep(-1)} 
                />
            }
            {step !== 5 && 
                    <input type='button' value='NEXT >>' 
                        className='foot-btn right-4'
                        onClick={() => moveStep(1)} 
                    />
            }
        </div>
    );
}