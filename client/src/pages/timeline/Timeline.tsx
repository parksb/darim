import React from 'react';
import styled from 'styled-components';

import { Section } from '../../components';
import List from "./List";

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const Timeline: React.FC = () => {
  return <Container>
    <List />
  </Container>
};

export default Timeline;
