# SonarCloud Quick Fix Guide

## ⚠️ The "Could not find a default branch" Error

This error occurs because SonarCloud hasn't analyzed your project yet and doesn't know which branch is the main one.

---

## ✅ Solution - Configure Default Branch in SonarCloud

### Method 1: Through SonarCloud Web UI (Easiest)

1. **Go to your project:** https://sonarcloud.io/project/configuration?id=JeetKarena_ZFish

2. **Navigate to Branches:**
   - Click on **"Branches"** in the left sidebar
   - Or go to: https://sonarcloud.io/project/branches?id=JeetKarena_ZFish

3. **Set Main Branch:**
   - Look for the branch named `main`
   - If it's not there yet, you may need to trigger the first analysis manually
   - Click on the branch settings
   - Set it as the "Main Branch" or "Default Branch"

4. **Alternative - Project Settings:**
   - Go to **Administration** → **Branches**
   - Set `main` as the reference branch

### Method 2: Trigger First Analysis Manually

If the branch doesn't appear yet:

1. **Go to Actions tab:** https://github.com/JeetKarena/ZFish/actions

2. **Find "SonarCloud Analysis" workflow**

3. **Click "Run workflow"**
   - Select branch: `main`
   - Click **"Run workflow"** button

4. **Wait for it to complete**

5. **Go back to SonarCloud** and refresh - the branch should now appear

---

## 🔄 What I Fixed

### 1. Updated SonarCloud Action
Changed from deprecated action to the new one:
```yaml
# Old (deprecated)
uses: SonarSource/sonarcloud-github-action@master

# New (current)
uses: SonarSource/sonarqube-scan-action@v5
```

### 2. Removed Branch Configuration
Removed this from `sonar-project.properties`:
```properties
# This was causing issues
sonar.branch.name=main
```

SonarCloud will auto-detect the branch from Git now.

### 3. Added Badges to README
Added 5 SonarCloud badges:
- Quality Gate Status
- Security Rating
- Maintainability Rating
- Coverage
- Bugs

---

## 🚀 Next Steps

1. **Set default branch in SonarCloud** (see Method 1 above)

2. **Or manually trigger the workflow** (see Method 2 above)

3. **Once the first analysis completes:**
   - The badges in README will start showing real data
   - Future pushes will automatically analyze
   - PRs will get SonarCloud comments

---

## 📊 Expected Results

After the first successful analysis, you'll see:

- ✅ **Quality Gate:** Pass/Fail status
- ✅ **Security Rating:** A-E rating
- ✅ **Maintainability Rating:** A-E rating
- ✅ **Coverage:** Test coverage percentage
- ✅ **Bugs:** Number of bugs detected

---

## 🐛 Still Having Issues?

### If workflow fails again:

1. **Check SONAR_TOKEN secret:**
   - Go to https://github.com/JeetKarena/ZFish/settings/secrets/actions
   - Verify `SONAR_TOKEN` exists and is correct

2. **Verify project key:**
   - In `sonar-project.properties`: `sonar.projectKey=JeetKarena_ZFish`
   - In SonarCloud, it should match exactly

3. **Check organization:**
   - In `sonar-project.properties`: `sonar.organization=jeetkarena`
   - In SonarCloud, verify this is your organization key

4. **Review logs:**
   - Go to Actions tab
   - Click on failed workflow
   - Check the "SonarCloud Scan" step for errors

---

**The workflow should work once the default branch is configured in SonarCloud!** 🎉
