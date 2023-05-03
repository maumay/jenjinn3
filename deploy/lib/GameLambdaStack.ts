import {CargoBinNames, LambdaType} from "./cargo";
import * as path from "path";
import {aws_iam as iam, aws_lambda as lambda, Stack} from "aws-cdk-lib";
import {Construct} from "constructs";
import {AccountAndRegion, LambdaConfig} from "../config";

export class GameLambdaStack extends Stack {

    readonly functionArn: string

    constructor(
        scope: Construct,
        id: string,
        accountAndRegion: AccountAndRegion,
        lambdaConfig: LambdaConfig,
        botFunctionNames: string[]
    ) {
        super(scope, id, {env: accountAndRegion});
        const cargoBinName = CargoBinNames.get(LambdaType.LichessGame)!
        const fn = new lambda.DockerImageFunction(this, id, {
            functionName: id,
            retryAttempts: 0,
            memorySize: lambdaConfig.memoryMB,
            timeout: lambdaConfig.timeout,
            code: lambda.DockerImageCode.fromImageAsset(
                path.join(__dirname, "..", ".."),
                {
                    file: path.join("tools", "workspace.dockerfile"),
                    buildArgs: {
                        APP_NAME: cargoBinName,
                        APP_CONFIG: ""
                    },
                }
            ),
        });
        const ps = new iam.PolicyStatement();
        ps.addActions("lambda:InvokeFunction");
        const {region, account} = accountAndRegion;
        const fnPrefix = `arn:aws:lambda:${region}:${account}:function`;
        ps.addResources(...botFunctionNames.map((bot) => `${fnPrefix}:${bot}`))
        fn.addToRolePolicy(ps);
        this.functionArn = fn.functionArn
    }
}
