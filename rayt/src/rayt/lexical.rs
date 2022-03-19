use crate::rayt::*;

enum Material_number {
    metal = 0,
    lambertian = 1,
    dielectric = 2,
    light = 3,
    any = 255,
}

enum Shape_number {
    sphere = 0,
    rect_xy = 1,
    rect_xz = 2,
    rect_yz = 3,
    box3d = 4,
    any = 255,
}

pub struct Laxial{
    contents:String,
    shape_number:Vec<u8>,
    material_number:Vec<u8>,
    begin:Vec<u64>,
    len:Vec<u64>,
    arg:Vec<f64>,
}

impl Laxial {
    pub fn new(contents:String) -> Self {
        let mut c:String = contents.clone();
        let mut cc :Vec<char> = c.chars().collect();
        let mut sn :Vec<u8> = Vec::<u8>::new();
        let mut mn :Vec<u8> = Vec::<u8>::new();
        let mut b  :Vec<u64> = Vec::<u64>::new();
        let mut l  :Vec<u64> = Vec::<u64>::new();
        let mut a:Vec<f64> = Vec::<f64>::new();
        let mut open:bool = false;
        let mut close:bool = false;
        let mut last:usize = 0;
        let mut next:bool = false;
        for i in 0..c.len(){
            if cc[i] == '('{
                if open{
                    next = true;
                }
                open = true;
            }else if cc[i] == ')'{
                if close{
                    next = true;
                }
                close = true;
            }else if cc[i] == '\n'{
                if open && close && !next{
                    let mut ar:String = "".to_string();
                    for k in last..i{
                        if cc[k] == '('{
                            if ar == "sphere"{
                                sn.push(0);
                            }else if ar == "rect_xy"{
                                sn.push(1);
                            }else if ar == "rect_xz"{
                                sn.push(2);
                            }else if ar == "rect_yz"{
                                sn.push(3);
                            }else if ar == "box3d"{
                                sn.push(4);
                            }else{
                                break;
                            }
                            ar  = "".to_string();
                        }else if cc[k] == ')'{
                        }else if cc[k] == ','{
                            if sn.len() != mn.len(){
                                if ar == "metal"{
                                    mn.push(0);
                                }else if ar == "lambertian"{
                                    mn.push(1);
                                }else if ar == "dielectric"{
                                    mn.push(2);
                                }else if ar == "light"{
                                    mn.push(3);
                                }else{
                                    mn.push(1);
                                    l.push(0);
                                    b.push(a.len() as u64);
                                }
                            }else{
                                let temp:f64 = ar.parse().unwrap();
                                a.push(temp);
                                l[0] += 1;
                            }
                            ar  = "".to_string();
                        }else{
                            ar.push(cc[k]);
                        }
                    }
                }
                open = false;
                close = false;
                next = false;
            }
        }
        Self { contents: contents,shape_number: sn,material_number: mn,begin:b,len:l,arg:a }
    }

    pub fn len(&self) -> usize { self.shape_number.len() }
    pub fn getContents(self) -> String { self.contents }
    pub fn getShape_number(&self,i:usize) -> u8 { self.shape_number[i] }
    pub fn getMaterial_number(&self,i:usize) -> u8 { self.material_number[i] }
    pub fn getArgi(&self,i:usize) -> f64{ self.arg[i] }
    pub fn getBegin(&self,i:usize) -> u64 { self.begin[i] }
    pub fn getLen(&self,i:usize) -> u64 { self.len[i] }
    pub fn getArg(&self,i:usize) -> Vec<f64>{
        let mut temp:Vec<f64> = Vec::<f64>::new();
        let sn = self.getShape_number(i);
        let b = self.getBegin(i);
        if sn == 0{
            //Color,Pos3,size
            for k in 0..7{
                temp.push(0.0);
            }
        }else if sn >= 1 && sn <= 3{
            //Color,Pos5
            for k in 0..8{
                temp.push(0.0);
            }
        }else{
            //Color,Pos6
            for k in 0..9{
                temp.push(0.0);
            }
        }
        let mn = self.getMaterial_number(i);
        if mn != 1{
            // 1
            temp.push(0.0);
        }
        for k in 0..self.getLen(i){
            if temp.len() <= k as usize{
                break;
            }
            let x = self.getArgi((k+b) as usize);
            temp[k as usize] = x;
        }
        temp
    }
}