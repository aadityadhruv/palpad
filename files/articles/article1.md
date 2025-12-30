# Big Header

I have a bunch of code to show:


Here's how you **convert** a directory. 

```
+  40 fn convert_dir(dir: &Path) {
+  41     if dir.is_dir() {
+  42         for entry: Result<DirEntry, Error> in fs::read_dir(path: dir).unwrap() {
+  43             let entry: DirEntry = entry.unwrap();
+  44             let path: PathBuf = entry.path();
+  45             if path.is_dir() {
+  46                 convert_dir(&path);
+  47             } else {
+  48                 convert_file(filepath: path);
+  49             }
+  50         }
+  51     }
+  52 }
```
