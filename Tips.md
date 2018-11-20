### <font color=red size=6 >1</font>.move in the directory in which the file is located. Input （<font color=green size=6 >rustc ***.rs</font>）， then input （<font color=green size=6 >windows:./xx.exe; ubuntu: ./***</font>）.[error:The term 'qwe.exe' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.]


### <font color=red size=6 >2</font>.Remainder symbols: % not %%



### <font color=red size=6 >3</font>.output  should be directly use （expression） not (variable = expression）.[error[E0308]: mismatched types//error[E0277]: cannot mod `f64` by `{integer}`]

### <font color=red size=6 >4</font>.when perform cargo file,must first use code: <font color=green size=6>cargo run</font>.
### not <font color=purple size=6>rustc ***.rs. </font>

### <font color=red size=6 >5</font>.How to use FLS to push large file to github.

Managing Large Files / Installing Git Large File Storage
 

### <font color=red>Installing Git Large File Storage</font>
MAC WINDOWS LINUX
In order to use Git LFS, you'll need to download and install a new program that's separate from Git.

Navigate to git-lfs.github.com and click Download.

Tip: For more information about alternative ways to install Git LFS for Linux, see this Getting started guide.

On your computer, locate and unzip the downloaded file.
Open Terminal.

Change the current working directory into the folder you downloaded and unzipped.

<font color=green>cd ~/Downloads/git-lfs-1.X.X</font>

Note: The file path you use after cd depends on your operating system, Git LFS version you downloaded, and where you saved the Git LFS download.

To install the file, run this command:

<font color=green>./install.sh</font>

<font color=yellow>Git LFS initialized.</font>

Note: You may have to use sudo ./install.sh to install the file.

Verify that the installation was successful:

<table>
<font color=green>git lfs install</font>

<font color=yellow>Git LFS initialized</font>
</table>

If you don't see a message indicating that git lfs install was successful, please contact GitHub Support or GitHub Premium Support. Be sure to include the name of your operating system.

### <font color=red>Configuring Git Large File Storage</font>

MAC WINDOWS LINUX

Once Git LFS is installed, you need to associate it with a large file in your repository.

If there are existing files in your repository that you'd like to use GitHub with, you need to first remove them from the repository and then add them to Git LFS locally. For more information, see "Moving a file in your repository to Git LFS."

If there are referenced Git LFS files that did not upload successfully, you will recieve an error message. For more information, see "Resolving Git Large File Storage upload failures."

Open Terminal.

Change your current working directory to an existing repository you'd like to use with Git LFS.

To associate a file type in your repository with Git LFS, enter git lfs track followed by the name of the file extension you want to automatically upload to Git LFS.

For example, to associate a .psd file, enter the following command:

<font color=green>git lfs track "*.psd"</font>

<font color=yellow>Adding path *.psd</font>

Every file type you want to associate with Git LFS will need to be added with git lfs track. This command amends your repository's .gitattributes file and associates large files with Git LFS.

Tip: We strongly suggest that you commit your local .gitattributes file into your repository. Relying on a global .gitattributes file associated with Git LFS may cause conflicts when contributing to other Git projects.

Add a file to the repository matching the extension you've associated:

<font color=green>
git add path/to/file.psd

git add .gitattributes
</font>

Commit the file and push it to GitHub:

<font color=green>
git commit -m "add file.psd"

git rm --cached /Users/Dora/Desktop/XXX/XXX/libbaiduNaviSDK.a

git commit --amend -CHEAD

git push origin master
</font>

You should see some diagnostic information about your file upload:

<font color=yellow>
Sending file.psd

44.74 MB / 81.04 MB  55.21 % 14s

64.74 MB / 81.04 MB  79.21 % 3s</font>

## Hint
<font size=1>同样是记录一下自己工作遇到的问题，免得下次再遇到了还到处网上查资料解决。

自己的项目的版本控制用的是Git，代码仓库在github托管。项目里用到了百度导航SDK，由于百度导航SDK有了新版本，于是就更新到了新版本，更新好了之后想把代码push到github上，结果出错了，被拒绝，具体信息是：Total 3007 (delta 664), reused 0 (delta 0)
remote: error: GH001: Large files detected.
remote: error: Trace: 7b7de6b9372ee392e0f3961b05ea6f33
remote: error: See http://git.io/iEPt8g for more information.
remote: error: File  XXX/XXX/BaiduNaviSDK/libbaiduNaviSDK.a is 102.68 MB; this exceeds GitHub's file size limit of 100.00 MB
To https://github.com/XXX/XXXX.git。意思是有大文件，更多信息可到http://git.io/iEPt8g查看，文件libbaiduNaviSDK.a的大小超过了GitHub限制的100M大小。想要push，必须把这个文件移除，可是要怎么移除呢？开始我是想着直接删除掉libbaiduNaviSDK.a，结果不行，还是会报上面的错，于是我又把libbaiduNaviSDK.a放到忽略文件里，结果还是不行，还是说有大文件，报同样的错。这下只能老老实实的看官方的解决办法了，于是就打开http://git.io/iEPt8g老老实实的研究了一番。大意是说为了便于管理代码库和方便合作伙伴们使用，当你push50M以上的文件时github将会警告，当push100M以上文件，就直接拒绝你的push，要想push必须把该文件从本地仓库和远程仓库全部移除掉。这个移除会永久性的从本地git和github里移除，如果这个文件很重要，要记得做一次备份。下面进入具体操作。如果这个文件是最近一次commit的，并且你还没有push到github，那么第一步输入命令 

<font color=green>cd /Users/Dora/Desktop/XXX</font>

（cd后面的这个路径要换成你自己项目的路径），然后第二步输入命令 

<font color=green>git rm --cached /Users/Dora/Desktop/XXX/XXX/libbaiduNaviSDK.a</font>

（加下划线部分是你自己的要移除的文件的路径），第三步输入命令 

<font color=green>git commit --amend -CHEAD</font>

，执行完这步后，这个大文件将会从你的commit记录里移除，并且以后commit都将不会再把它commit了，这时候就可以git push把本地代码push到github上了。

注意：这里可能遇到输入git rm --cached /Users/Dora/Desktop/XXX/XXX/libbaiduNaviSDK.a命令后说找不到你要删除的文件的问题，出现这个问题的原因是你要删除的文件路径名没写对，一定要仔细检查，确保要删除的文件的路径是正确的。

如果做了这几步你push的时候还是报和开始的时候一样的错，那说明这个文件你不是最近一次commit时添加的，而是在之前commit过很多次了，这就需要把关于这个文件的所有历史commit记录全部清除掉，这时候我们就需要用到一个叫BFG的工具。我们要到https://rtyley.github.io/bfg-repo-cleaner/#download这个网站去下载并学习如何使用这个工具。首先先得把这个软件下载下来，双击它，如果你的电脑没有java的SDK，按照提示安装好，否则BFG将运行不了。好了之后，我们要cd进BFG文件所在目录，第一步输入命令java -jar bfg.jar --no-blob-protection --strip-blobs-bigger-than 50M my-repo.git（红色部分是你下载下来的文件的名字,蓝色部分是你需要移除的文件大小，橙色部分是你自己.git文件的路径），第二步cd my-repo.git ，第三步 git reflog expire --expire=now --all && git gc --prune=now --aggressive，第四步git push，到此大功告成。你的commit历史里所有大于50M的文件的commit全部被清除，这样你就可以push到github上了。这里只是记录了怎么处理遇到的问题，如果想要知道原理，则需要好好好去学习学习。

注意：这里可能遇到输入命令java -jar bfg.jar --strip-blobs-bigger-than 50M my-repo.git后bfg运行不成功的问题，原因是java -jar bfg.jar这个命令必须要在bfg.jar所在目录下运行才行，否则系统将不知道bfg.jar文件在哪里，所以执行不成功。所以我们要先cd到bfg.jar所在目录，然后在执行ava -jar bfg.jar --strip-blobs-bigger-than 50M my-repo.git命令。对于不熟悉命令行的人来说最容易犯这些错误了。</font>

Links:

1.https://help.github.com/articles/configuring-git-large-file-storage/
2.http://jingpin.jikexueyuan.com/article/36116.html
