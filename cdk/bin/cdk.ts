#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { CdkStack } from '../lib/cdk-stack';

const app = new cdk.App();
// eslint-disable-next-line no-new
new CdkStack(app, 'CdkStack', {
    env: { region: process.env.AWS_REGION ?? 'eu-west-1' }
});
