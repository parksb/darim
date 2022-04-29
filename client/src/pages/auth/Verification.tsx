import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import ReCAPTCHA from 'react-google-recaptcha';
import Secret from '../../utils/secret';
import Storage from '../../utils/storage';

import * as api from '../../api/user';
import { getI18n } from '../../utils/i18n';
import {
  Button, Checkbox, Container, TextField, Section,
} from '../../components';
import {
  localStoragePrivateKey, reCAPTCHASiteKey, profile, Profile,
} from '../../constants';

interface Props {
  tokenKey: string;
  privateKeyState: [string, React.Dispatch<React.SetStateAction<string>>];
  publicKeyState: [string, React.Dispatch<React.SetStateAction<string>>];
}

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const LinkSpan = styled.span`
  margin-left: 5px;
`;

const StyledLink = styled(Link)`
  color: #000000;
  font-weight: bold;

  &:hover {
    background-color: #ffce05;
  }
`;

const Verification: React.FC<Props> = ({ tokenKey, privateKeyState, publicKeyState }) => {
  const [hasAgreed, setHasAgreed] = useState(false);
  const [reCAPTCHAToken, setReCAPTCHAToken] = useState('');

  const [pin, setPin] = useState('');
  const setPrivateKey = privateKeyState[1];
  const setPublicKey = publicKeyState[1];

  const verify = async () => {
    if (hasAgreed) {
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
    termsOfService: {
      ko: '서비스 이용약관',
      en: 'Terms of Service',
    },
    privacyPolicy: {
      ko: '개인정보처리방침',
      en: 'Privacy Policy',
    },
    agreeTermsAndPrivacy: {
      ko: '서비스 이용약관과 개인정보처리방침을 모두 확인하였으며, 동의합니다.',
      en: 'Check here to indicate that you have read and agree to the Terms of Service and Privacy Policy.',
    },
  });

  return <Container>
    <Section>{i18n.text('info')}</Section>
    <Section top={20} row>
      <Checkbox text={i18n.text('agreeTermsAndPrivacy')} valueState={[hasAgreed, setHasAgreed]} />
      <LinkSpan>
        (<StyledLink target='_blank' to='/static/terms'>{i18n.text('termsOfService')}</StyledLink>, <StyledLink target='_blank' to='/static/privacy'>{i18n.text('privacyPolicy')}</StyledLink>)
      </LinkSpan>
    </Section>
    <Section top={20}>
      {profile === Profile.PRODUCTION ? <ReCAPTCHA sitekey={reCAPTCHASiteKey} onChange={(value) => (value && setReCAPTCHAToken(value))} /> : <label><input type='checkbox' onChange={() => { setReCAPTCHAToken('token'); }} />reCAPTCHA</label>}
    </Section>
    <Section top={30}>
      <Section>{i18n.text('verificationGuide')}</Section>
      <Section top={10} row>
        <FullWidthTextField type='text' placeholder={i18n.text('pin')} value={pin} onChange={({ target: { value } }) => setPin(value)} />
        <Button onClick={verify} disabled={!hasAgreed || !reCAPTCHAToken}>{i18n.text('verify')}</Button>
      </Section>
    </Section>
  </Container>;
};

export default Verification;
