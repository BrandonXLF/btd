---
title: Build. Transform. Deploy.
sidebar_label: btd
description: Transform and deploy production builds of projects with file operations and commands using easy to write YAML files.
---

# btd

<div style={{fontSize: '1.75rem'}}>Build. Transform. Deploy.</div>

---

`btd` allows you to transform and deploy production builds of projects with file operations and commands using easy-to-edit YAML files. It supports Unix-like and Windows servers.

## Convenient File Format

Transformations are stored in [instruction files](/file-format/) that are created using the easy-to-edit YAML file format. YAML makes it trivial to define multiline strings strings wile requiring minimal syntax.

## User-Wide Store

Instructions can be located in codebase or in a [user-wide library](/the-library/), located anywhere, that allows for the separation of open-source projects from the specific instructions for deploying them to proprietary environments.
