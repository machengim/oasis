import React, {useState, useEffect} from 'react';
import {useTranslation} from 'react-i18next';

export const LangContext = React.createContext({
    language: 'en',
    setLanguage: () => {},
});

export default function LangProvider(props) {
    const [language, setLanguage] = useState('en');
    const {i18n} = useTranslation();
    useEffect(() => {
        i18n.changeLanguage(language);
    }, [language]);

    return (
        <LangContext.Provider value={{language, setLanguage}}>
            { props.children }
        </LangContext.Provider>
    );
}   