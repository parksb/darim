import React, { useState } from 'react';
import styled from 'styled-components';
import { Secret, Storage } from 'snowball-js';
import ReCAPTCHA from 'react-google-recaptcha';

import * as api from '../../api/user';
import { getI18n } from '../../utils/i18n';
import { Button, Checkbox, Container, TextField, Section } from '../../components';
import { localStoragePrivateKey, reCAPTCHASiteKey } from '../../constants';

import privacy from '../../../public/static/privacy.html';
import terms from '../../../public/static/terms.html';

interface Props {
  tokenKey: string;
  privateKeyState: [string, React.Dispatch<React.SetStateAction<string>>];
  publicKeyState: [string, React.Dispatch<React.SetStateAction<string>>];
}

const Frame = styled(Section)`
  height: 415px;
  overflow-y: scroll;
  border: 1px solid #000;
  padding: 30px;
  max-width: 325px;
`;

const BoxContainer = styled(Section)`
  justify-content: space-between;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const Verification: React.FC<Props> = ({ tokenKey, privateKeyState, publicKeyState }) => {
  const [hasAgreedWithPrivacy, setHasAgreedWithPrivacy] = useState(false);
  const [hasAgreedWithTerms, setHasAgreedWithTerms] = useState(false);
  const [reCAPTCHAToken, setReCAPTCHAToken] = useState('');

  const [pin, setPin] = useState('');
  const setPrivateKey = privateKeyState[1];
  const setPublicKey = publicKeyState[1];

  const verify = async () => {
    if (hasAgreedWithPrivacy && hasAgreedWithTerms) {
      const generatedPublicKey = Secret.getRandomString();
      const generatedPrivateKey = Secret.getRandomString();
      const encryptedPrivateKey = Secret.encryptAES(generatedPrivateKey, generatedPublicKey);

      const result = await api.createUser(generatedPublicKey, tokenKey, pin, reCAPTCHAToken);
      if (result) {
        Storage.set(localStoragePrivateKey, encryptedPrivateKey);
        setPrivateKey(encryptedPrivateKey);
        setPublicKey(generatedPublicKey);
      }
    }
  };

  const i18n = getI18n({
    info: {
      ko: '️🚀 한 단계만 남았어요!',
      en: '🚀 Only one step left!',
    },
    verify: {
      ko: '인증 ↗',
      en: 'Verify ↗',
    },
    pin: {
      ko: '인증키',
      en: 'Key',
    },
    verificationGuide: {
      ko: '📧 이메일로 계정을 활성화할 수 있는 인증키가 발송되었습니다. 메일에 포함된 인증키를 복사, 붙여넣기해주세요.',
      en: '📧 The email containing a key to activate your account is sent. Please copy and paste the key',
    },
  });

  return <Container>
    <Section>{i18n.text('info')}</Section>
    <BoxContainer top={30} row>
      <Section>
        <Frame dangerouslySetInnerHTML={{__html: privacy}} />
        <Section top={10}>
          <Checkbox text='개인정보처리방침에 동의합니다.' valueState={[hasAgreedWithPrivacy, setHasAgreedWithPrivacy]} />
        </Section>
      </Section>
      <Section>
        <Frame dangerouslySetInnerHTML={{__html: terms}} />
        <Section top={10}>
          <Checkbox text='서비스 이용약관에 동의합니다.' valueState={[hasAgreedWithTerms, setHasAgreedWithTerms]} />
        </Section>
      </Section>
    </BoxContainer>
    <Section top={30}>
      <ReCAPTCHA sitekey={reCAPTCHASiteKey} onChange={(value) => { console.log(value); value && setReCAPTCHAToken(value)} } />
    </Section>
    <Section top={40}>
      <Section>{i18n.text('verificationGuide')}</Section>
      <Section top={10} row>
        <FullWidthTextField type='text' placeholder={i18n.text('pin')} value={pin} onChange={({ target: { value } }) => setPin(value)} />
        <Button onClick={verify} disabled={!hasAgreedWithTerms || !hasAgreedWithPrivacy || !reCAPTCHAToken}>{i18n.text('verify')}</Button>
      </Section>
    </Section>
  </Container>
};

export default Verification;
