use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Database {
    pub packages: Vec<Package>,
    pub last_updated: String,
}

impl Database {
    pub fn parse(db: &str) -> Database {
        let db: Vec<&str> = db.split("\\n").collect();
        let last_updated: String = Database::get_cur_time();
        let packages: Vec<Package> = db[0..].iter().map(|p| Package::parse(p)).collect();
        Database {
            packages: packages,
            last_updated: last_updated.to_string(),
        }
    }

    fn get_cur_time() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let cur_time = since_the_epoch.as_secs();
        cur_time.to_string()
    }

    pub fn find_package(&self, package: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == package)
    }
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub author: String,
    pub repo: String,
    pub dist: String,
    pub keywords: String,
}

impl Package {
    pub fn parse(package: &str) -> Package {
        let package: Vec<&str> = package.split(";;").collect();
        let name = package[0];
        let description = package[1];
        let author = package[2];
        let repo = package[3];
        let dist = package[4];
        let keywords = package[5];
        Package {
            name: name.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            repo: repo.to_string(),
            dist: dist.to_string(),
            keywords: keywords.to_string(),
        }
    }
}
