<div align="center">
	<img src="./assets/rscs_header_1.png" alt="" />
</div>

<div align="center">
   <p align="center">
      For use configuring machines in the West-Mec IT Security Repair Shop (Nerd Heard).
   </p>
</div>

<div align="center">
   <div id="toc">
      <ul style="list-style: none;">
         <summary>
            <h2 align="center">Dev Setup on Windows</h2>
         </summary>
      </ul>
   </div>
</div>

<ol>
   <li>
      <div>
         <a href="https://rustup.rs/">
            <span>Rustup</span>
            <img src="https://developer.fedoraproject.org/static/logo/rust.png" alt="" width=15 />
         </a>
         <ul>
            <li>
               <p>
                  Or run: <pre><code>winget install -e --id Rustlang.Rustup</code></pre>
               </p>
            </li>
         </ul>
      </div>
   </li>
   <li>
      <div>
         <p>
            <span>Visual Studio Build Tools</span>
            <img src="https://static.wikia.nocookie.net/windows/images/1/1c/Visual_Studio_Code_1.35_icon.png/revision/latest/thumbnail/width/360/height/360?cb=20231126223748" alt="" width=15 />
         </p>
         <ul>
            <li>
               <pre><code>winget install -e --source winget --id Microsoft.VisualStudio.2022.Community --silent --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.NativeDesktop --includeRecommended</code></pre>
            </li>
         </ul>
      </div>
   </li>
</ol>

<div align="center">
   <div id="toc">
      <ul style="list-style: none;">
         <summary>
            <h2 align="center">Dev Setup on Linux</h2>
         </summary>
      </ul>
   </div>
</div>

> [!TIP]
> After setting it up, you should also install `tokei` to see the repo's code stats.
> ```sh
> tokei .
> ```

<table align="center">
   <thead>
      <tr>
         <th width="500px">
            <b>Distro</b>
         </th>
         <th width="500px">
            <b>Command(s)</b>
         </th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td align="center" valign="center">
            <div id="toc">
               <ul style="list-style: none;">
                  <summary>
                     <h2 align="left">Arch</h2>
                     <img align="right" src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/13/Arch_Linux_%22Crystal%22_icon.svg/330px-Arch_Linux_%22Crystal%22_icon.svg.png" alt="" width=25 />
                  </summary>
               </ul>
            </div>
</td>
<td>
<pre><code>sudo paru -S mingw-w64-gcc rustup</code></pre>
</td>
</tr>
<tr>
<td align="center" valign="center">
<div id="toc" align="center">
<ul style="list-style: none;">
<summary>
<h2 align="left">Debian</h2>
<img align="right" src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/66/Openlogo-debianV2.svg/500px-Openlogo-debianV2.svg.png" alt="" width=25 />
</summary>
</ul>
</div>
</td>
<td>
<pre><code>sudo apt update
sudo apt install mingw-w64</code></pre>
</td>
      </tr>
      <tr>
         <td align="center" valign="center">
            <div id="toc">
               <ul style="list-style: none;">
                  <summary>
                     <h2 align="left">Fedora</h2>
                     <img align="right" src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Fedora_logo.svg/960px-Fedora_logo.svg.png" alt="" width=25 />
                  </summary>
               </ul>
            </div>
</td>
<td>
<pre><code>sudo dnf install mingw64-gcc</code></pre>
</td>
</tr>
</tbody>
</table>

## Builds

Run `build.*` depending on which OS you are on (Linux is `.sh`, and Windows is `.bat`).

### Release

Run `build-release.*` depending on which OS you are on (Linux is `.sh`, and Windows is `.bat`).
