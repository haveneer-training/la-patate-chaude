pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}

pub struct MonstrousMazeOutput {
    pub path: String,
}

fn monstrous_maze(input:MonstrousMazeInput)->MonstrousMazeOutput{
    let mut list=Vec::new();
    let mut list_first_directions=Vec::new();
    let mut grid2: String=input.grid.clone();//TEMP
    let mut endurance2=input.endurance;
    let mut path: String="".to_string();
    let mut c=0;
    let mut y=0;
    let mut p;
    let mut last_removed: char;
    while c<input.grid.len() {
        if input.grid.chars().nth(c).unwrap()=='\n' {
            break;
        }
        c+=1;
    }
    while y<input.grid.len() {
        if input.grid.chars().nth(y).unwrap()=='Y' {
            break;
        }
        y+=1;
    }
    if y==input.grid.len() {
        let output=MonstrousMazeOutput{
            path: "pas de solution".to_string(),
        };
        return output;
    }
    p=y;
    while input.grid.chars().nth(p).unwrap()!='X' {
        if (p==y)&&(!list_first_directions.contains(&'v'))&&((y+c+1)<input.grid.len())&&((input.grid.chars().nth(y+c+1).unwrap()==' ')||(input.grid.chars().nth(y+c+1).unwrap()=='X')||((input.grid.chars().nth(y+c+1).unwrap()=='M')&&(endurance2>1))) {
            if input.grid.chars().nth(y+c+1).unwrap()=='M' {
                endurance2-=1;
            }
            p=y+c+1;
            path="v".to_string();
            list_first_directions.push('v');
        }
        else if (p==y)&&(!list_first_directions.contains(&'>'))&&((y+1)<input.grid.len())&&((input.grid.chars().nth(y+1).unwrap()==' ')||(input.grid.chars().nth(y+1).unwrap()=='X')||((input.grid.chars().nth(y+1).unwrap()=='M')&&(endurance2>1))) {
            if input.grid.chars().nth(y+1).unwrap()=='M' {
                endurance2-=1;
            }
            p=y+1;
            path=">".to_string();
            list_first_directions.push('>');
        }
        else if (p==y)&&(!list_first_directions.contains(&'^'))&&((y-c-1)<input.grid.len())&&((input.grid.chars().nth(y-c-1).unwrap()==' ')||(input.grid.chars().nth(y-c-1).unwrap()=='X')||((input.grid.chars().nth(y-c-1).unwrap()=='M')&&(endurance2>1))) {
            if input.grid.chars().nth(y-c-1).unwrap()=='M' {
                endurance2-=1;
            }
            p=y-c-1;
            path="^".to_string();
            list_first_directions.push('^');
        }
        else if (p==y)&&(!list_first_directions.contains(&'<'))&&((y-1)<input.grid.len())&&((input.grid.chars().nth(y-1).unwrap()==' ')||(input.grid.chars().nth(y-1).unwrap()=='X')||((input.grid.chars().nth(y-1).unwrap()=='M')&&(endurance2>1))) {
            if input.grid.chars().nth(y-1).unwrap()=='M' {
                endurance2-=1;
            }
            p=y-1;
            path="<".to_string();
            list_first_directions.push('<');
        }
        if input.grid.chars().nth(p).unwrap()=='X' {
            let output=MonstrousMazeOutput{
                path: path,
            };
            return output;
        }
        if path.len()==0 {
            let output=MonstrousMazeOutput{
                path: "pas de solution".to_string(),
            };
            return output;
        }
        while (input.grid.chars().nth(p).unwrap()==' ')||(input.grid.chars().nth(p).unwrap()=='M') {
            grid2.replace_range(
                grid2
                    .char_indices()
                    .nth(p)
                    .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                    .unwrap(),
                "#",
            );//TEMP
            list.push(p);
            if path.chars().nth(path.len()-1).unwrap()=='v' {
                p=p+c+1;
                path=path+"v";
            }
            else if path.chars().nth(path.len()-1).unwrap()=='>' {
                p=p+1;
                path=path+">";
            }
            else if path.chars().nth(path.len()-1).unwrap()=='^' {
                p=p-c-1;
                path=path+"^";
            }
            else if path.chars().nth(path.len()-1).unwrap()=='<' {
                p=p-1;
                path=path+"<";
            }
            if list.contains(&p) {
                break;
            }
            if input.grid.chars().nth(p).unwrap()=='X' {
                let output=MonstrousMazeOutput{
                    path: path,
                };
                return output;
            }
            if (input.grid.chars().nth(p).unwrap()=='M')&&(endurance2>1) {
                endurance2-=1;
            }
            else if (input.grid.chars().nth(p).unwrap()=='M')&&(endurance2<=1) {
                break;
            }
        }
        if path.chars().nth(path.len()-1).unwrap()=='v' {
            p=p-c-1;
        }
        else if path.chars().nth(path.len()-1).unwrap()=='>' {
            p=p-1;
        }
        else if path.chars().nth(path.len()-1).unwrap()=='^' {
            p=p+c+1;
        }
        else if path.chars().nth(path.len()-1).unwrap()=='<' {
            p=p+1;
        }
        last_removed=path.chars().nth(path.len()-1).unwrap();
        path=(&path[..path.len()-1]).to_string();
        while path.len()>=1 {
            if (path.chars().nth(path.len()-1).unwrap()=='v')&&(last_removed=='v')&&(!list.contains(&(p-1)))&&((input.grid.chars().nth(p-1).unwrap()==' ')||(input.grid.chars().nth(p-1).unwrap()=='X')||((input.grid.chars().nth(p-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-1;
                path=path+"<";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='v')&&(last_removed=='v')&&(!list.contains(&(p+1)))&&((input.grid.chars().nth(p+1).unwrap()==' ')||(input.grid.chars().nth(p+1).unwrap()=='X')||((input.grid.chars().nth(p+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+1;
                path=path+">";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='>')&&(last_removed=='v')&&(!list.contains(&(p-c-1)))&&((input.grid.chars().nth(p-c-1).unwrap()==' ')||(input.grid.chars().nth(p-c-1).unwrap()=='X')||((input.grid.chars().nth(p-c-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-c-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-c-1;
                path=path+"^";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='>')&&(last_removed=='>')&&(!list.contains(&(p+c+1)))&&((input.grid.chars().nth(p+c+1).unwrap()==' ')||(input.grid.chars().nth(p+c+1).unwrap()=='X')||((input.grid.chars().nth(p+c+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+c+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+c+1;
                path=path+"v";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='>')&&(last_removed=='>')&&(!list.contains(&(p-c-1)))&&((input.grid.chars().nth(p-c-1).unwrap()==' ')||(input.grid.chars().nth(p-c-1).unwrap()=='X')||((input.grid.chars().nth(p-c-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-c-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-c-1;
                path=path+"^";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='^')&&(last_removed=='>')&&(!list.contains(&(p-1)))&&((input.grid.chars().nth(p-1).unwrap()==' ')||(input.grid.chars().nth(p-1).unwrap()=='X')||((input.grid.chars().nth(p-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-1;
                path=path+"<";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='^')&&(last_removed=='^')&&(!list.contains(&(p+1)))&&((input.grid.chars().nth(p+1).unwrap()==' ')||(input.grid.chars().nth(p+1).unwrap()=='X')||((input.grid.chars().nth(p+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+1;
                path=path+">";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='^')&&(last_removed=='^')&&(!list.contains(&(p-1)))&&((input.grid.chars().nth(p-1).unwrap()==' ')||(input.grid.chars().nth(p-1).unwrap()=='X')||((input.grid.chars().nth(p-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-1;
                path=path+"<";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='<')&&(last_removed=='^')&&(!list.contains(&(p+c+1)))&&((input.grid.chars().nth(p+c+1).unwrap()==' ')||(input.grid.chars().nth(p+c+1).unwrap()=='X')||((input.grid.chars().nth(p+c+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+c+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+c+1;
                path=path+"v";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='<')&&(last_removed=='<')&&(!list.contains(&(p-c-1)))&&((input.grid.chars().nth(p-c-1).unwrap()==' ')||(input.grid.chars().nth(p-c-1).unwrap()=='X')||((input.grid.chars().nth(p-c-1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p-c-1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p-c-1;
                path=path+"^";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='<')&&(last_removed=='<')&&(!list.contains(&(p+c+1)))&&((input.grid.chars().nth(p+c+1).unwrap()==' ')||(input.grid.chars().nth(p+c+1).unwrap()=='X')||((input.grid.chars().nth(p+c+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+c+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+c+1;
                path=path+"v";
                break;
            }
            else if (path.chars().nth(path.len()-1).unwrap()=='v')&&(last_removed=='<')&&(!list.contains(&(p+1)))&&((input.grid.chars().nth(p+1).unwrap()==' ')||(input.grid.chars().nth(p+1).unwrap()=='X')||((input.grid.chars().nth(p+1).unwrap()=='M')&&(endurance2>1))) {
                if input.grid.chars().nth(p+1).unwrap()=='M' {
                    endurance2-=1;
                }
                p=p+1;
                path=path+">";
                break;
            }
            else{
                let index=list.iter().position(|x| *x == p).unwrap();
                list.remove(index);
                grid2.replace_range(
                    grid2
                        .char_indices()
                        .nth(p)
                        .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                        .unwrap(),
                    " ",
                );//TEMP
                if input.grid.chars().nth(p).unwrap()=='M' {
                    endurance2+=1;
                    grid2.replace_range(
                        grid2
                            .char_indices()
                            .nth(p)
                            .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                            .unwrap(),
                        "M",
                    );//TEMP
                }
                if path.chars().nth(path.len()-1).unwrap()=='v' {
                    p=p-c-1;
                }
                else if path.chars().nth(path.len()-1).unwrap()=='>' {
                    p=p-1;
                }
                else if path.chars().nth(path.len()-1).unwrap()=='^' {
                    p=p+c+1;
                }
                else if path.chars().nth(path.len()-1).unwrap()=='<' {
                    p=p+1;
                }
                last_removed=path.chars().nth(path.len()-1).unwrap();
                path=(&path[..path.len()-1]).to_string();
            }
        }
    }
    let output=MonstrousMazeOutput{
        path: path,
    };
    return output;
}