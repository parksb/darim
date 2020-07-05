import styled from 'styled-components'
import React from 'react';

import { Section } from '../components/index';

const Container = styled(({ fullWidth, top, bottom, ...other }) => <Section {...other} />)`
  width: 100%;
  max-width: ${props => props.fullWidth ? 'none' : '800px'};
  margin-top: ${props => props.top ? `${props.top}px` : 'auto'};
  margin-bottom: ${props => props.bottom ? `${props.bottom}px` : 'auto'};
  margin-right: auto;
  margin-left: auto;
`;

export default Container;
