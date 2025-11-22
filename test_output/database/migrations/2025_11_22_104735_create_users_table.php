<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('users', function (Blueprint $table) {
            
                        $table->integer('id');
            $table->string('name', 255);
            $table->string('email', 255)->unique();
            $table->string('password', 255);
            $table->string('bio', 255)->nullable();
            $table->string('avatarUrl', 255)->nullable();
            $table->integer('links');
            $table->integer('categories');

            $table->timestamps();
            $table->softDeletes();
        });

        
    }

    public function down(): void
    {
        Schema::dropIfExists('users');
    }
};
