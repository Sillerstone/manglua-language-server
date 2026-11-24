import * as path from "path";
import * as fs from "fs";
import * as vscode from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from "vscode-languageclient/node";

const MANGLUA_EXTENSION = "mlua";

let client: LanguageClient | undefined;

function resolveServerCommand(context: vscode.ExtensionContext): string {
    const config = vscode.workspace.getConfiguration("mangLua");
    const configuredPath = config.get<string>("serverPath")?.trim();
    if (configuredPath) {
        return configuredPath;
    }

    const exeName = process.platform === "win32" ? "manglua-ls.exe" : "manglua-ls";

    const devBinary = path.resolve(
        context.extensionPath,
        "..",
        "manglua-ls",
        "target",
        "debug",
        exeName
    );
    if (fs.existsSync(devBinary)) {
        return devBinary;
    }

    const bundledBinary = context.asAbsolutePath(path.join("server", exeName));
    if (fs.existsSync(bundledBinary)) {
        return bundledBinary;
    }

    // Retrieve from PATH
    return exeName;
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    const command = resolveServerCommand(context);

    const serverOptions: ServerOptions = {
        run: {command, transport: TransportKind.stdio},
        debug: {command, transport: TransportKind.stdio},
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{scheme: "file", language: MANGLUA_EXTENSION}],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher(`**/*.${MANGLUA_EXTENSION}`),
        },
    };

    client = new LanguageClient(
        "mangLuaLS",
        "MangLua Language Server",
        serverOptions,
        clientOptions
    );

    await client.start();
}

export async function deactivate(): Promise<void> {
    if (client) {
        await client.stop();
        client = undefined;
    }
}