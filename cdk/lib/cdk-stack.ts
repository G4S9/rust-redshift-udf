import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as iam from 'aws-cdk-lib/aws-iam';
import { type Construct } from 'constructs';
import * as path from 'node:path';

const INDEX_NAME = process.env.INDEX_NAME;

export class CdkStack extends cdk.Stack {
    constructor (scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        if(!INDEX_NAME) {
            throw new Error('INDEX_NAME env var missing');
        }

        const rustRedshiftUdf = new lambda.Function(this, 'RustLambdaFunction', {
            functionName: 'rust-redshift-udf',
            code: lambda.Code.fromAsset(path.join(__dirname, '../../lambda/target/lambda/rust-redshift-udf/')),
            handler: 'ignored_for_bootstrapped_images',
            runtime: lambda.Runtime.PROVIDED_AL2,
            memorySize: 128,
            environment: {
                INDEX_NAME,
            }
        });

        rustRedshiftUdf.role?.attachInlinePolicy(
            new iam.Policy(this, 'LocationSearchPolicy', {
                statements: [new iam.PolicyStatement({
                    actions: ['geo:SearchPlaceIndexForText'],
                    resources: ['*']
                })]
            })
        );
    }
}
