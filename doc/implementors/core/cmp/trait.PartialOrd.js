(function() {var implementors = {};
implementors["hal"] = [{"text":"impl&lt;S:&nbsp;PartialOrd&gt; PartialOrd&lt;Frame&lt;S&gt;&gt; for Frame&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: FrameSize,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;PartialOrd + FrameSize&gt; PartialOrd&lt;Page&lt;S&gt;&gt; for Page&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;PhysicalAddress&gt; for PhysicalAddress","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;VirtualAddress&gt; for VirtualAddress","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Size4KiB&gt; for Size4KiB","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Size2MiB&gt; for Size2MiB","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Size1GiB&gt; for Size1GiB","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;PciAddress&gt; for PciAddress","synthetic":false,"types":[]}];
implementors["kernel"] = [{"text":"impl PartialOrd&lt;KernelObjectId&gt; for KernelObjectId","synthetic":false,"types":[]}];
implementors["libpebble"] = [{"text":"impl PartialOrd&lt;PciAddress&gt; for PciAddress","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Handle&gt; for Handle","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl PartialOrd&lt;Level&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Level&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;Metadata&lt;'a&gt;&gt; for Metadata&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;MetadataBuilder&lt;'a&gt;&gt; for MetadataBuilder&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl PartialOrd&lt;Ident&gt; for Ident","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl PartialOrd&lt;Lifetime&gt; for Lifetime","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()