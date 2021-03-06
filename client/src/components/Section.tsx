import React from 'react';
import styled from 'styled-components'

const Section = styled(({ row, top, bottom, fullHeight, nowrap, ...other }) => <div {...other} />)`
  display: flex;
  flex: ${props => !props.row && props.fullHeight && 1};
  flex-wrap: ${props => props.nowrap ? 'nowrap' : 'wrap'};
  flex-direction: ${props => props.row ? 'row' : 'column'};
  max-width: 100%;
  margin-top: ${props => props.top ? `${props.top}px` : 0};
  margin-bottom: ${props => props.bottom ? `${props.bottom}px` : 0};
  line-height: 150%;
`;

export default Section;
