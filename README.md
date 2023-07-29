<h1 align='center'>Portal Finder⛏</h1>
<h3 align='center'>This is a hobby project that allows you to find an End portal using a minimal amount of Eyes of Ender</h3>
<h2>🎮How to Use?</h2>
<ol>
<li>Throw an Eye of Ender from <b>any</b> location</li><br><br>
<li>Align the cursor directly with the stopped Eye of Ender</li><br><br>
 
![Общий вид](https://github.com/muminovbob/portal_finder/raw/main/images/1.png)
![Общий вид](https://github.com/muminovbob/portal_finder/raw/main/images/2.png)
<li>Record the results in the program</li><br><br>
<li>Repeat the steps from a different location, moving away from the previous location by a significant distance (the further the more accurate, 500+ blocks in any axis)</li><br><br>
</ol>
<h2>👨‍🔬How does it work?</h2>
<p>The program creates two linear equations from the entered data. The point of intersection of these functions represents the coordinates of the portal. The point of intersection is determined by solving the system of equations using the Cramer's rule with matrices. </p>
<p>By examining the code, you may notice that the slope (k) in the equation y = kx + b is calculated as the tangent of the angle (a) plus 90 degrees. Why? The X-axis in Minecraft is unique. It belongs to (♾, -♾) instead of the other way around. Hence, some tricks have to be performed.</p>

<h2>💾Executable</h2>
<h3>🦀Rust</h3>
<i>Size: 239 KB</i>

<ul>
 <li><code>git clone https://github.com/muminovbob/portal_finder.git</code></li>
 <li><code>cd portal_finder/rust</code></li>
 <li><code>cargo run</code></li>
</ul>
<p><br></p>

<h3>🟦Python</h3>
<i>Size: 5.73 MB</i><br>
There is a third-party library called <a href='https://pypi.org/project/python-cfonts/'>cfonts</a> used for console styling. So, if you use pyinstaller, make sure to include the venv/lib/cfonts directory.
<p><br></p>
<ul>
 <li><code>git clone https://github.com/muminovbob/portal_finder.git</code></li>
 <li><code>cd portal_finder/python</code></li>
 <i>Create/enter into a venv</i>
 <li><code>pip install -r requirements.txt</code></li>
 <li><code>python main.py</code></li>
</ul>
<hr>
<p align=center><i>🌟It was exciting to develop this project!
 Feel free to contribute <a href='https://github.com/muminovbob/portal_finder/issues'>here</a></i></p>
