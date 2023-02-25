# scmd

Command runner that can specify options in shorthand form

## Examples

For config file below (placed at `${HOME}/.config/scmd/config.toml` for Linux, `${HOME}/Library/Application Support/rs.scmd/config.toml` for macOS),

```toml:config.toml
[default]
passthrough_unknown_command = false
placeholder = '{}'

[command.curl]
# always save xattr
base = ['curl', '--xattr']

[command.curl.headparams]
json = ['-H', 'Content-Type: application/json']

[command.curl.middleparams]
example-api = ['--insecure', 'http://example.com/path/to/api-base/{}']
```

the command

```
scmd curl json example-api=some/endpoint -- -d '{}' -o output.json
```

will expanded and executed as below.

```
curl -H 'Content-Type: application/json' --xattr --insecure http://example.com/path/to/api-base/some/endpoint -d '{"foo": "bar"}' -o otput.json
```

## Other explanations

WIP
