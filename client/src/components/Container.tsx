import styled from 'styled-components'
import React from 'react';

import { Section } from '../components/index';

const Container = styled(({ fullWidth, fullHeight, top, bottom, ...other }) => <Section {...other} />)`
  display: flex;
  width: 100%;
  flex: ${props => !props.row && props.fullHeight && 1};
  height: ${props => props.fullHeight ? '100%' : 'auto'};
  max-width: ${props => props.fullWidth ? 'none' : '800px'};
  margin-top: ${props => props.top ? `${props.top}px` : 0};
  margin-bottom: ${props => props.bottom ? `${props.bottom}px` : 0};
  margin-right: auto;
  margin-left: auto;
`;

export default Container;
