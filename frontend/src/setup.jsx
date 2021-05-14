import React, {useState, useContext} from 'react';
import ReactDom from 'react-dom';
import LanguageProvider, { LangContext } from './contexts/language';
import { useTranslation } from 'react-i18next';
import './i18n/config';
import Step1 from './components/setup/step1';
import Step2 from './components/setup/step2';
import Step3 from './components/setup/step3';
import Step4 from './components/setup/step4';
import Step5 from './components/setup/step5';

ReactDom.render(
    <App />, 
    document.getElementById('root')
);

function App() {
    const [step, setStep] = useState(2);
    const [language, setLanguage] = useState('en');
    const [username, setUsername] = useState('admin');
    const [password, setPassword] = useState('');
    const [allowGuest, setAllowGuest] = useState(1);
    const [groups, setGroups] = useState(initGroups());

    function initGroups() {
        return [{name: "Admin", power: 9, default: false, delete: false, editPower: false, editDefault: false},
                {name: "User", power: 3, default: true, delete: false, editPower: true, editDefault: true}];
    }

    function checkStepComplete() {
        if (step === 1) {
            if (username.trim().length > 0 && password.trim().length > 0) {
                return true;
            } else {
                alert("Username and password cannot be empty!");
                return false;
            }
        }

        return true;
            
    }

	return (
        <LanguageProvider>
            <div className='w-full h-full bg-white'>
                <div className='w-4/5 lg:w-1/2 mx-auto text-center'>
                    <Title />
                    {step == 1 && <Step1 
                        language={language} setLanguage={setLanguage}
                        username={username} setUsername={setUsername}
                        password={password} setPassword={setPassword}
                        allowGuest={allowGuest} setAllowGuest={setAllowGuest} 
                    />}
                    {step == 2 && <Step2 groups={groups} setGroups={setGroups} />}
                    {step == 3 && <Step3 />}
                    {step == 4 && <Step4 />}
                    {step == 5 && <Step5 />}
                    <FootButtons step={step} setStep={setStep} 
                        checkStepComplete={checkStepComplete}
                    />
                </div>
            </div>
        </LanguageProvider>
    );
}

function Title() {
    const { t } = useTranslation();
    const langContext = useContext(LangContext);

    return (
        <div className='text-center text-3xl font-bold mt-12'>
            {t('OASIS Setup')}
        </div>
    );
}

function FootButtons(props) {
    const step = props.step;
    const setStep = props.setStep;
    const checkStepComplete = props.checkStepComplete;

    function moveStep(i) {
        if (i < 0 || checkStepComplete()) {
            setStep(step + i);
        }
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