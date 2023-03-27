import{h as d,j as p,k as u,al as _,c as m,am as h,m as n,an as t,ao as o,A as s,F as f,ap as v,aq as g,ar as b,s as l,as as x,at as y,q as k,au as N,av as w,_ as P}from"./nav-ee82deb8.js";import{N as S}from"./NoteDisplay-f570d18f.js";import{u as V}from"./index-38fd116c.js";import"./_commonjsHelpers-725317a4.js";const j={class:"m-4"},L={class:"mb-10"},T={class:"text-4xl font-bold mt-2"},B={class:"opacity-50"},C={class:"text-lg"},D={class:"font-bold flex gap-2"},H={class:"opacity-50"},q=t("div",{class:"flex-auto"},null,-1),z={key:0,class:"border-gray-400/50 mb-8"},A=d({__name:"PresenterPrint",setup(F){p(u),_(`
@page {
  size: A4;
  margin-top: 1.5cm;
  margin-bottom: 1cm;
}
* {
  -webkit-print-color-adjust: exact;
}
html,
html body,
html #app,
html #page-root {
  height: auto;
  overflow: auto !important;
}
`),V({title:`Notes - ${m.title}`});const i=h(()=>b.slice(0,-1).map(a=>{var r;return(r=a.meta)==null?void 0:r.slide}).filter(a=>a!==void 0&&a.noteHTML!==""));return(a,r)=>(l(),n("div",{id:"page-root",style:g(s(w))},[t("div",j,[t("div",L,[t("h1",T,o(s(m).title),1),t("div",B,o(new Date().toLocaleString()),1)]),(l(!0),n(f,null,v(s(i),(e,c)=>(l(),n("div",{key:c,class:"flex flex-col gap-4 break-inside-avoid-page"},[t("div",null,[t("h2",C,[t("div",D,[t("div",H,o(e==null?void 0:e.no)+"/"+o(s(x)),1),y(" "+o(e==null?void 0:e.title)+" ",1),q])]),k(S,{"note-html":e.noteHTML,class:"max-w-full"},null,8,["note-html"])]),c<s(i).length-1?(l(),n("hr",z)):N("v-if",!0)]))),128))])],4))}}),$=P(A,[["__file","/home/runner/work/ll-udp-pubsub/ll-udp-pubsub/slidev/node_modules/@slidev/client/internals/PresenterPrint.vue"]]);export{$ as default};
