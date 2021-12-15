use dirs::home_dir;
use std::{
    path::Path,
    fs::{
        rename,
        ReadDir,
        create_dir,
        remove_file
    }
};


pub fn recycle_bin_list() -> ReadDir {
    let trash = Path::new(&home_dir().unwrap()).join("trash");
    return trash.read_dir().unwrap();
}


pub fn recycel_check() -> (){
    let trash = Path::new(&home_dir().unwrap()).join("trash");
    if !trash.exists() {
        create_dir(&trash).unwrap();
    }
    
}

pub fn recycle_bin_empty() -> () {
    recycle_bin_list().for_each(|entry| {
        let path = entry.unwrap().path();
        remove_file(&path).unwrap();
    });
}


pub struct Trash {
    pub path: String
}

impl Trash {
    pub fn check(&self,current: bool ) -> bool {
        if current == false {
            let trash = Path::new(&home_dir().unwrap()).join("trash");
            let file_name = Path::new(self.path.as_str()).file_name().unwrap().to_str().unwrap();
            let new_path = trash.join(file_name);
            return Path::new(&new_path).exists();
        }
        return Path::new(self.path.as_str()).exists();
    }
    pub fn file_move(&self) -> () {
        let trash = Path::new(&home_dir().unwrap()).join("trash");
        let file_name = Path::new(self.path.as_str()).file_name().unwrap().to_str().unwrap();
        let new_path = trash.join(file_name);
        rename(self.path.as_str(), &new_path).unwrap();
    }

    pub fn recycle_bin_delete(&self) -> () {
        let trash = Path::new(&home_dir().unwrap()).join("trash");
        let file_name = Path::new(self.path.as_str()).file_name().unwrap().to_str().unwrap();
        let new_path = trash.join(file_name);
        remove_file(&new_path).unwrap();
    }


}
