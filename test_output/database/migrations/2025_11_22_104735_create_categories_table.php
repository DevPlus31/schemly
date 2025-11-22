<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('categories', function (Blueprint $table) {
            
                        $table->integer('id');
            $table->integer('userId');
            $table->string('name', 255);
            $table->integer('user');
            $table->integer('links');

            $table->timestamps();
            $table->softDeletes();
        });

        
    }

    public function down(): void
    {
        Schema::dropIfExists('categories');
    }
};
