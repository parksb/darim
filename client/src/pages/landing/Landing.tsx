import React from 'react';
import styled from 'styled-components';

import { getI18n } from '../../utils/i18n';
import { Container, Section } from '../../components';
import Login from './Login'
import { Session } from '../../models';
import { Join } from '../auth';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const SectionTitle = styled.h2`
  font-size: 22px;
  font-weight: bold;
  color: #303030;
`;

const SectionContent = styled.p`
  margin-top: 20px;
`;

const Image = styled.img`
  width: 100%;
`;

const Landing: React.FC<Props> = ({ session_state }) => {
  const i18n = getI18n({
    keepYourDiaryASecret: {
      ko: '🔏 다이어리의 비밀을 지키세요',
      en: '🔏 Keep your diary a secret'
    },
    keepYourDiaryASecretDescription: {
      ko: '암호화를 통해 다이어리 내용을 보호할 수 있습니다. 심지어 개발자도 알아낼 수 없습니다.',
      en: 'You can keep your diary a secret via encryption. Even the developer can\'t figure out it',
    },
    youCanUseVariousFormatsUsingMarkdown: {
      ko: '🛠 마크다운을 이용해 다양한 형식을 사용할 수 있어요',
      en: '🛠 You can use various formats using markdown',
    },
    youCanUseVariousFormatsUsingMarkdownDescription: {
      ko: '마크다운 문법을 통해 코드 하이라이팅, 테이블, 각주, KaTeX 등 다양한 형식을 사용할 수 있습니다.',
      en: 'You can use code highlighting, table, footnotes, KaTeX, and more via markdown syntax.',
    },
    getStartedNow: {
      ko: '👋 지금 시작해보세요!',
      en: '👋 Get started now!',
    }
  });

  return <Container>
    <Section>
      <Login session_state={session_state} />
      <Section>
        <Section top={30}>
          <Image src='https://user-images.githubusercontent.com/6410412/87238882-579d4900-c443-11ea-8e81-267b3243237c.png' />
        </Section>
        <Section top={40}>
          <SectionTitle>{i18n.text('keepYourDiaryASecret')}</SectionTitle>
          <SectionContent>{i18n.text('keepYourDiaryASecretDescription')}</SectionContent>
        </Section>
        <Section top={50}>
          <SectionTitle>{i18n.text('youCanUseVariousFormatsUsingMarkdown')}</SectionTitle>
          <SectionContent>{i18n.text('youCanUseVariousFormatsUsingMarkdownDescription')}</SectionContent>
        </Section>
        <Section top={50}>
          <SectionTitle>{i18n.text('getStartedNow')}</SectionTitle>
          <Section top={20}>
            <Join />
          </Section>
        </Section>
      </Section>
    </Section>
  </Container>
};

export default Landing;
