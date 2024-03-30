[Setup]
AppName = btd
AppId = C2378321-DDC9-448B-971C-E77C98816ED2
AppVersion = {#GetFileVersion("target\release\btd.exe")}
DefaultDirName = {autopf}\btd
OutputBaseFilename = btd-installer
PrivilegesRequired = lowest
ArchitecturesInstallIn64BitMode = x64
ChangesEnvironment = yes
OutputDir = target\release
AppPublisher = Brandon Fowler

[Components]
Name: path; Description: Add to user $PATH; Types: full

[Files]
Source: "target\release\btd.exe"; DestDir: "{app}\bin"; DestName: btd.exe
Source: "LICENSE"; DestDir: "{app}"

[Dirs]
Name: "{app}\bin"

[Registry]
Root: HKCU; Subkey: "Environment"; \
	ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Components: path; \
	Check: PathNeeded()

[Run]
Filename: "https://brandonxlf.github.io/btd/"; Description: "Open online documentation"; \
	Flags: shellexec runasoriginaluser postinstall unchecked;

[Code]
function PathNeeded(): Boolean;
var Path: String;
begin
	if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then begin
		Result := True;
		exit;
	end;
	
	Result := Pos(ExpandConstant(';{app}\bin;'), ';' + Path + ';') = 0;
end;

procedure RemovePath();
var Match: Integer; Path: String;
begin
    if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then exit;

    Match := Pos(ExpandConstant(';{app}\bin;'), ';' + Path + ';');
	if Match = 0 then exit;

    Delete(Path, Match - 1, Length(ExpandConstant('{app}\bin')) + 1);
    RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
    if CurUninstallStep<>usPostUninstall then Exit;
    RemovePath();
end;
