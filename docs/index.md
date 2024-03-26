# Transform Deploy

<div style={{fontSize: '1.35rem'}}>Production build to deployment file transformer</div>

---

Transform production builds of projects using file operations and commands into a deployment-ready state forms and deploy them.

## Convenient File Format

Transformations are stored in [Instruction Files](file-format) that are created using the easy to edit YAML file format. YAML makes it trivial to define multiline strings strings wile requiring minimal syntax.

## Computer-Wide Store

Instructions can be in codebase or in a [computer-wide library](the-library) that allows for the separation of open-source projects from the specific instructions for deploying it to proprietary environments.
