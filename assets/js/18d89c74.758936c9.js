"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[705],{1966:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>o,contentTitle:()=>s,default:()=>h,frontMatter:()=>t,metadata:()=>d,toc:()=>c});var r=i(4848),l=i(8453);const t={},s="File Format",d={id:"file-format",title:"File Format",description:"Instruction Files are YAML files consisting of a list of Transformations. To learn more about the YAML file format, visit yaml.org.",source:"@site/../docs/file-format.md",sourceDirName:".",slug:"/file-format",permalink:"/btd/file-format",draft:!1,unlisted:!1,editUrl:"https://github.com/brandonxlf/btd/tree/main/website/../docs/file-format.md",tags:[],version:"current",frontMatter:{},sidebar:"docSidebar",previous:{title:"Installation",permalink:"/btd/install"},next:{title:"Running Instructions",permalink:"/btd/running"}},o={},c=[{value:"Transformations",id:"transformations",level:2},{value:"<code>meta</code>",id:"meta",level:3},{value:"<code>run</code>",id:"run",level:3},{value:"<code>create</code>",id:"create",level:3},{value:"<code>replace</code>",id:"replace",level:3},{value:"<code>prepend</code>",id:"prepend",level:3},{value:"<code>append</code>",id:"append",level:3},{value:"<code>rename</code>",id:"rename",level:3},{value:"<code>copy</code>",id:"copy",level:3},{value:"<code>delete</code>",id:"delete",level:3},{value:"<code>deploy</code>",id:"deploy",level:3}];function a(e){const n={a:"a",code:"code",em:"em",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",ul:"ul",...(0,l.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.h1,{id:"file-format",children:"File Format"}),"\n",(0,r.jsxs)(n.p,{children:["Instruction Files are YAML files consisting of a list of ",(0,r.jsx)(n.a,{href:"#transformations",children:"Transformations"}),". To learn more about the YAML file format, visit ",(0,r.jsx)(n.a,{href:"https://yaml.org/",children:"yaml.org"}),"."]}),"\n",(0,r.jsx)(n.h2,{id:"transformations",children:"Transformations"}),"\n",(0,r.jsxs)(n.p,{children:["Each Transformation is a YAML dictionary with a ",(0,r.jsx)(n.code,{children:"type"})," key corresponding to one of the options below."]}),"\n",(0,r.jsx)(n.h3,{id:"meta",children:(0,r.jsx)(n.code,{children:"meta"})}),"\n",(0,r.jsx)(n.p,{children:"The first entry. Contains information about the Instruction File."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"dir"})," string - Base directory to use for commands and file operations. All relatives paths are processed relative to this path."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"run",children:(0,r.jsx)(n.code,{children:"run"})}),"\n",(0,r.jsx)(n.p,{children:"Run a command."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"cmd"})," string - The command to run."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"cwd"})," (",(0,r.jsx)(n.em,{children:"optional"}),") string - The current working directory to run the command in. Defaults to the ",(0,r.jsx)(n.code,{children:"meta"})," transformation's ",(0,r.jsx)(n.code,{children:"dir"}),"."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"env"})," (",(0,r.jsx)(n.em,{children:"optional"}),") string: string map - Mapping of environment variables to set."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"create",children:(0,r.jsx)(n.code,{children:"create"})}),"\n",(0,r.jsx)(n.p,{children:"Create a file with content."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"file"})," string - The file to create."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"text"})," string - Text to create the file with."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"replace",children:(0,r.jsx)(n.code,{children:"replace"})}),"\n",(0,r.jsx)(n.p,{children:"Find and replace text in a file."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"file"})," string - The file to replace text in."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"find"})," string - The text to find. Interpreted as normal text unless ",(0,r.jsx)(n.code,{children:"regex"})," is ",(0,r.jsx)(n.code,{children:"true"}),"."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"replace"})," string - The text to replace. If ",(0,r.jsx)(n.code,{children:"regex"})," is ",(0,r.jsx)(n.code,{children:"true"}),", substitutions (eg. ",(0,r.jsx)(n.code,{children:"$1"}),", ",(0,r.jsx)(n.code,{children:"$2"}),", etc.) are supported."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"regex"})," (",(0,r.jsx)(n.em,{children:"optional"}),") boolean - Interpret ",(0,r.jsx)(n.code,{children:"find"})," as a regular expression. Default is ",(0,r.jsx)(n.code,{children:"false"}),"."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"prepend",children:(0,r.jsx)(n.code,{children:"prepend"})}),"\n",(0,r.jsx)(n.p,{children:"Find and replace text in a file."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"file"})," string - The file to prepend text to."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"text"})," string - Text to prepend the file with."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"append",children:(0,r.jsx)(n.code,{children:"append"})}),"\n",(0,r.jsx)(n.p,{children:"Append text to the end of a file."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"file"})," string - The file to append to text."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"text"})," string - Text to append the file with."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"rename",children:(0,r.jsx)(n.code,{children:"rename"})}),"\n",(0,r.jsx)(n.p,{children:"Rename a file or directory."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"from"})," string - The old file path."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"to"})," string - The new file path."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"copy",children:(0,r.jsx)(n.code,{children:"copy"})}),"\n",(0,r.jsxs)(n.p,{children:["Copy a file. To copy a directory, use ",(0,r.jsx)(n.code,{children:"scp"}),"."]}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"from"})," string - The path of the original file."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"to"})," string - The path of the copy to create."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"delete",children:(0,r.jsx)(n.code,{children:"delete"})}),"\n",(0,r.jsx)(n.p,{children:"Delete a file or directory."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"file"})," string - The file or directory to delete."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"recursive"})," (",(0,r.jsx)(n.em,{children:"optional"}),") boolean - Delete items recursively if ",(0,r.jsx)(n.code,{children:"file"})," is a directory. Default is ",(0,r.jsx)(n.code,{children:"false"}),"."]}),"\n"]}),"\n",(0,r.jsx)(n.h3,{id:"deploy",children:(0,r.jsx)(n.code,{children:"deploy"})}),"\n",(0,r.jsx)(n.p,{children:"Deploy a file or directory to a production environment via secure copy."}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"from"})," string - The local directory/file to copy from."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"to"})," string - The production directory/file to copy to as an ",(0,r.jsx)(n.code,{children:"scp"})," path."]}),"\n",(0,r.jsxs)(n.li,{children:[(0,r.jsx)(n.code,{children:"clear"})," (",(0,r.jsx)(n.em,{children:"optional"}),") boolean - Remove ",(0,r.jsx)(n.code,{children:"to"})," before replacing it with ",(0,r.jsx)(n.code,{children:"from"}),". Useful for directories. Files are transferred to the remote server before ",(0,r.jsx)(n.code,{children:"to"})," is removed. Default is ",(0,r.jsx)(n.code,{children:"false"}),"."]}),"\n"]})]})}function h(e={}){const{wrapper:n}={...(0,l.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(a,{...e})}):a(e)}},8453:(e,n,i)=>{i.d(n,{R:()=>s,x:()=>d});var r=i(6540);const l={},t=r.createContext(l);function s(e){const n=r.useContext(t);return r.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(l):e.components||l:s(e.components),r.createElement(t.Provider,{value:n},e.children)}}}]);