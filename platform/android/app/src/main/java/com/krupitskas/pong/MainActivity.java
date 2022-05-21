package com.krupitskas.pong;

import androidx.appcompat.app.AppCompatActivity;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.security.crypto.MasterKeys;

import android.content.SharedPreferences;
import android.graphics.Bitmap;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.TextView;
import android.widget.Toast;

import java.io.IOException;
import java.security.GeneralSecurityException;

public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        String r = RustBindings.greeting("Android world");
        ((TextView)findViewById(R.id.helloWorldText)).setText(r);

        Bitmap bitmap = Bitmap.createBitmap(800, 800, Bitmap.Config.ARGB_8888);
        RustBindings.renderFractal(bitmap);
        ImageView imageView = findViewById(R.id.imageView);
        imageView.setImageBitmap(bitmap);

        findViewById(R.id.bImportKeys).setOnClickListener(v -> {
            Log.d("lustig", "import keys");
            saveKey("savinhg a new key here if this works -- would ya look at thag");
        });


        findViewById(R.id.bExportKeys).setOnClickListener(v -> {
            String key = readKey();
            Toast.makeText(MainActivity.this, key, Toast.LENGTH_LONG).show();
        });

    }

    private void saveKey(String key) {
        String masterKeyAlias = null;
        try {
            masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC);

            SharedPreferences sharedPreferences = EncryptedSharedPreferences.create(
                    "secret_shared_prefs",
                    masterKeyAlias,
                    MainActivity.this,
                    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
                    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
            );

            // use the shared preferences and editor as you normally would
            SharedPreferences.Editor editor = sharedPreferences.edit();

            editor.putString("oursecretkey", key);
            editor.apply();
        } catch (GeneralSecurityException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private String readKey() {
        Log.d("zzz lustig", "readkey");
        String masterKeyAlias = null;

        try {
            masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC);

            SharedPreferences sharedPreferences = EncryptedSharedPreferences.create(
                    "secret_shared_prefs",
                    masterKeyAlias,
                    MainActivity.this,
                    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
                    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
            );


            return sharedPreferences.getString("oursecretkey", "nothing was there");
        } catch (GeneralSecurityException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }


        return "an exception happened because you doint know what your'e doing";
    }
}

