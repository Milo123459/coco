# coco
Curl scripts made simple

## Install

Windows

```
scoop bucket add cone https://github.com/Milo123459/cone
scoop install cone/coco
```

Linux/MacOS

```
curl -fsSL https://raw.githubusercontent.com/Milo123459/coco/master/install.sh | bash
```

## Concept

You can add scripts with arguments, meaning you will never forget a curl script again!

## Usage

```
# Add a script
coco add NAME SCRIPT
# Run a script
coco NAME
# If the name is generic (ie, cmds, action, list, etc):
coco run NAME
# List all scripts
coco list
```

## Formatting

Arguments are prefixed with + and then you provide argument numbers, starting from 0. For example:
+1 would get first argument, +2 second and +3+ will get all arguments after argument number 3 (inclusive).

## Example

```
coco add contributions "https://github-contributions-api.deno.dev/$1.term"
coco contributions
# Error! Argument 1 was not provided
coco contributions Milo123459
... the response
```