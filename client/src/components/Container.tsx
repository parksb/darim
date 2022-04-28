import styled from 'styled-components'

import { Section, SectionProps } from '../components';

export interface ContainerProps extends SectionProps {
  fullWidth?: boolean;
}

export const Container = styled(Section)<ContainerProps>`
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

