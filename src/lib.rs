extern crate regex;

use neon::prelude::*;
use regex::Regex;
use std::{
    fs,
    io::{self, Write},
};

struct Tangent {
    // raw: String,
    start: usize,
    end: usize,
    replacement: String,
}

struct ShuffleOperator {
    // id: i32,
    // plus: bool,
    cipher: i32,
    // operator: String,
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("read_file_to_string", read_file_to_string)?;
    cx.export_function("deobfuscate_scopes", deobfuscate_scopes)?;
    Ok(())
}

fn read_file_to_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let path_prm = cx.argument::<JsString>(0)?;
    let path_str = path_prm.value(&mut cx).to_string();
    let contents = fs::read_to_string(path_str).expect("Error: Unable to read the file.");
    Ok(cx.string(contents))
}

fn deobfuscate_scopes(mut cx: FunctionContext) -> JsResult<JsString> {
    let raw_js_prm = cx.argument::<JsString>(0)?;
    let raw_js_str = raw_js_prm.value(&mut cx).to_string();
    let mut transformer = JsModuleTransformer::new(raw_js_str);
    match transformer.transform() {
        Ok(()) => Ok(cx.string(&transformer.result)),
        Err(e) => panic!("Failed to transform: {}", e),
    }
}

// fn save_to_file(filename: &str, content: &str) -> io::Result<()> {
//     // Create a new file or truncate it if it already exists
//     let mut file = fs::File::create(filename)?;

//     // Write the content to the file
//     file.write_all(content.as_bytes())?;

//     Ok(())
// }

struct JsModuleTransformer {
    raw: String,
    result: String,
    tangent_cipher: i32,
    agents: Vec<String>,
    // bosses: Vec<String>,
    origins: Vec<String>,
    tangents: Vec<Tangent>,
}

impl JsModuleTransformer {
    fn new(raw: String) -> Self {
        // let origin_path = "src/origins.txt";
        // let origins_text = fs::read_to_string(origin_path).expect("Failed to read file");
        let origins_text="at,boolean,nextSibling,random,anonymous,status,human,chlApiLanguage,display,responseText,managed,charCode,cached,add,translations,string,redirecting,message,invalid,atob,open,contentinfo,createObject,parent,touchstart,floor,metadata,interactive,splice,footer,chlApiOverrunBudget,turnstile,catch,min,center,set,isArray,readyState,appendChild,insertBefore,send,cloudflare,favicon,rtl,map,span,challenge,dir,chlApiWidget,style,msg,darkmode,alt,pow,pop,feedbackInit,overlay,check,toLowerCase,length,protocol,cursor,classList,url,onerror,chlApiOverrideDarkmode,test,wheel,from,class,pointer,zone,prototype,role,getResponseHeader,keys,page,replace,value,self,function,location,height,grid,crossorigin,time,click,padStart,isExtension,script,timeout,onclick,setAttribute,fromCharCode,cookie,call,ltr,visible,chlApiUrl,href,firstElementChild,body,push,bind,stack,index,flex,setRequestHeader,passive,column,now,event,console,onreadystatechange,cookies,line,log,innerText,pointerover,hidden,text,keydown,cookieEnabled,chlApiRumWidgetAge,reload,outdated,setTimeout,encode,loading,object,isSecureContext,lang,src,parseInt,substring,includes,split,block,startsWith,polyfills,subtle,getPrototype,top,errorInfoObject,err,then,join,hasOwnProperty,diagnostic,img,source,clearfix,navigator,inline,core,abs,overrun,this,slice,performance,ontimeout,digest,createTextNode,inner,chlApiSitekey,font,concat,widget,addEventListener,matchMedia,success,auto,stringify,browser,review,chlApiClientVersion,sort,revokeObject,match,https,window,title,parentNode,parse,apply,symbol,createElement,getOwnPropertyNames,to,postMessage,spinner,pointermove,hostname,number,application,content,heading,error,getTime,input,querySelector,terminate,lip,spacer,button,toString,removeChild,flow,document,visibility,address,bigint,mousemove,chlApi,setTime,alignItems,justifyContent,margin,head,type,matches";
        JsModuleTransformer {
            raw,
            result: String::new(),
            tangent_cipher: 0,
            agents: Vec::new(),
            // bosses: Vec::new(),
            origins: origins_text
                .split(",")
                .map(|s| s.to_string())
                .collect(),
            tangents: Vec::new(),
        }
    }

    fn transform(&mut self) -> Result<(), String> {
        self.agents = self.get_agent_list()?;
        // self.bosses = self.get_boss_list()?;
        self.tangent_cipher = self.get_tangent_cipher()?;
        self.agents = self.get_shuffle_operators()?;
        self.tangents = self.catch_tangents()?;
        self.result = self.replace_tangents();
        Ok(())
    }

    fn get_agent_list(&self) -> Result<Vec<String>, String> {
        let agent_list_re =
            Regex::new(r"return \w+='([^'~]+(~[^'~]+)*)'").expect("Invalid regex pattern");
        if let Some(captures) = agent_list_re.captures(&self.raw) {
            if let Some(matched) = captures.get(1) {
                // let mut origins: Vec<&str> = vec![];
                // let origin_re =
                //     Regex::new(r"[~'][a-z]{2,}([A-Z][a-z]{2,})*").expect("Invalid regex pattern");
                // for mat in origin_re.find_iter(matched.as_str()) {
                //     if !origins.contains(&mat.as_str()) {
                //         origins.push(mat.as_str());
                //     }
                // }
                // let _ = save_to_file("origins.txt", &origins.join(",\n"));
                return Ok(matched.as_str().split("~").map(|s| s.to_string()).collect());
            }
        }
        Err("Cannot find agent list.".to_string())
    }

    // fn get_boss_list(&self) -> Result<Vec<String>, String> {
    //     let boss_list_re = Regex::new(r"B='([a-z](A[a-z])*)'").expect("Invalid regex pattern");
    //     if let Some(captures) = boss_list_re.captures(&self.raw) {
    //         if let Some(matched) = captures.get(1) {
    //             return Ok(matched.as_str().split("A").map(|s| s.to_string()).collect());
    //         }
    //     }
    //     Err("Cannot find boss list.".to_string())
    // }

    fn get_tangent_cipher(&self) -> Result<i32, String> {
        let tangent_cipher_re = Regex::new(r"f=f-(\d+)").expect("Invalid regex pattern");
        if let Some(captures) = tangent_cipher_re.captures(&self.raw) {
            // Access the first capturing group
            if let Some(matched) = captures.get(1) {
                match matched.as_str().parse::<i32>() {
                    Ok(number) => return Ok(number),
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err("Cannot get tangent cipher.".to_string())
    }

    fn get_shuffle_operators(&self) -> Result<Vec<String>, String> {
        let shuffle_operators_re =
            Regex::new(r"(([+\*])?)\(?(-?)parseInt\(\w+\((\d+)\)\)/((\d+))").unwrap();
        let shuffle_keys_re = Regex::new(r"~(([1-9]\d*)[a-zA-Z]+)~").unwrap();
        let mut operators = Vec::new();

        for cap in shuffle_operators_re.captures_iter(&self.raw) {
            // let operator = &cap[1];
            let cipher: i32 = cap[4].parse().unwrap();
            // let id: i32 = cap[5].parse().unwrap();
            // let plus = cap[3].is_empty() || &cap[3] == "+";

            operators.push(ShuffleOperator {
                // id,
                // plus,
                cipher,
                // operator: operator.to_string(),
            });
        }
        operators.sort_by(|a, b| a.cipher.cmp(&b.cipher));
        let key_count = operators.len();
        let mut correct_key: i32 = 0;
        for j in 0..=key_count {
            let mut key_flaw: Vec<i32> = vec![];
            for (i, cap) in shuffle_keys_re.captures_iter(&self.raw).enumerate() {
                let real = &cap[1];
                // let key: i32 = cap[2].parse().unwrap();
                let number: i32 = self.agents.iter().position(|x| x == real).unwrap() as i32
                    + self.tangent_cipher;
                let shuffle_offset = (operators[(i + j) % key_count].cipher - number
                    + self.agents.len() as i32)
                    % self.agents.len() as i32;

                // println!(
                //     "{}: {} - {} = {}",
                //     j,
                //     operators[(i + j) % key_count].cipher,
                //     number,
                //     shuffle_offset
                // );

                if !key_flaw.contains(&shuffle_offset) {
                    key_flaw.push(shuffle_offset);
                }
            }
            if key_flaw.len() == 1 {
                correct_key = key_flaw[0];
            }
        }
        let mut shuffled_agents = self.agents.clone();
        println!("Correct key: {}", correct_key);
        for _ in 1..=correct_key {
            if let Some(last) = shuffled_agents.pop() {
                shuffled_agents.insert(0, last);
            }
        }
        Ok(shuffled_agents)
    }

    fn catch_tangents(&self) -> Result<Vec<Tangent>, String> {
        let tangent_re = Regex::new(r"[^\w\d]\w{2}\((\d{3,4})\)").expect("Invalid regex pattern");
        let allowed_before_agent_char = Regex::new(r"[^()=\n]").expect("Invalid regex pattern");
        let allowed_after_agent_char = Regex::new(r"[\w\d]").expect("Invalid regex pattern");
        let mut tangents = vec![];

        for mat in tangent_re.find_iter(&self.raw) {
            let raw = mat.as_str();
            let mut start = mat.start() + 1;
            let mut end = mat.end();

            if let Some(captures) = tangent_re.captures(raw) {
                if let Some(matched) = captures.get(1) {
                    match matched.as_str().parse::<i32>() {
                        Ok(index) => {
                            // Calculate the replacement based on the tangent_cipher
                            let replacement_index = index - self.tangent_cipher;
                            if replacement_index >= 0
                                && (replacement_index as usize) < self.agents.len()
                            {
                                let mut replacement =
                                    self.agents[replacement_index as usize].clone();
                                let is_origin = start > 0
                                    && self.raw.get(start - 1..start).map(|s| s == "[")
                                        == Some(true)
                                    && allowed_before_agent_char
                                        .is_match(self.raw.get(start - 2..start - 1).unwrap())
                                    && self.origins.contains(&replacement);
                                let need_space = allowed_after_agent_char
                                    .is_match(self.raw.get(start..start + 1).unwrap());
                                if is_origin && need_space {
                                    replacement = String::from(".") + &replacement + " ";
                                    start = start - 1;
                                    end = end + 1;
                                    // raw = &self.raw[start..end]
                                } else if is_origin {
                                    replacement = String::from(".") + &replacement;
                                    start = start - 1;
                                    end = end + 1;
                                } else {
                                    replacement = String::from("'") + &replacement + "'";
                                }
                                tangents.push(Tangent {
                                    // raw: raw.to_string(),
                                    start,
                                    end,
                                    replacement: replacement,
                                });
                                // println!(
                                //     "Arrested: {} -> {}",
                                //     self.raw[start..end].to_owned(),
                                //     replacement
                                // );
                            } else {
                                if let Some(full_match) = captures.get(0) {
                                    println!("Skipped: {}", full_match.as_str());
                                }
                            }
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
        }
        Ok(tangents)
    }

    fn replace_tangents(&self) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        // Iterate over the tangents and build the final string
        for tangent in &self.tangents {
            // Append the part of the raw string before the tangent
            result.push_str(&self.raw[last_end..tangent.start]);
            // Append the replacement
            result.push_str(&tangent.replacement);
            // Update last_end to the end of the current tangent
            last_end = tangent.end;
        }

        // Append the remaining part of the raw string after the last tangent
        result.push_str(&self.raw[last_end..]);
        result
    }
}
