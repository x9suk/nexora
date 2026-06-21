package nexora;

import com.intellij.lexer.FlexLexerAdapter;

public class NexoraLexerAdapter extends FlexLexerAdapter {
    public NexoraLexerAdapter() {
        super(new NexoraLexer());
    }
}
