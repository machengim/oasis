import React, {useState} from 'react';
import {Container, Row, Col, Button} from 'react-bootstrap';
import Step1 from './components/Step1';
import Step2 from './components/Step2';
import './index.css';
import 'bootstrap/dist/css/bootstrap.min.css';

type TitleProps = {
    text: string;
}

type SubTitleProps = {
    text: string;
    step: number;
}

type NavButtonsProps = {
    step: number;
    setStep: Function;
}

export default function App() {
    const [step, setStep] = useState(2);

    return (
      <Container>
        <Title text={"Oasis Settings"}  />
        <SubTitle text={"Admin Account"} step={step} />
        {step === 1 && <Step1 />}
        {step === 2 && <Step2 />}
        <NavButton step={step} setStep={setStep} />
      </Container>
    );
}
  
function Title(title: TitleProps) {
  return (
    <Row className="justify-content-md-center my-2">
      <Col md="auto">
        <h1>{title.text}</h1>
      </Col>
    </Row>
  );
}

function SubTitle(subtitle: SubTitleProps) {
    const texts = ["Admin Account", "User Groups"];
    const step = subtitle.step;
    const sub = texts[step - 1];

    return (
        <Row className="justify-content-md-center my-2">
            <Col md="auto">
                <h3>{sub}</h3>
            </Col>
        </Row>
    );
}
  
function NavButton(btnCtrl: NavButtonsProps) {

  function changeStep(i: number) {
      let newStep = btnCtrl.step + i;
      if (newStep < 1) newStep = 1;
      else if (newStep > 5) newStep = 5;

      btnCtrl.setStep(newStep);
  }

  return (
    <div className="box mt-5">
      <div className="flex">
        <Button variant="primary" onClick={() => changeStep(-1)}>Prev</Button>
        <Button variant="primary" onClick={() => changeStep(1)}>Next</Button>
      </div>
    </div>
  );
}
