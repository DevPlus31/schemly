<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('links', function (Blueprint $table) {
            
                        $table->integer('id');
            $table->integer('userId');
            $table->integer('categoryId')->nullable();
            $table->string('title', 255);
            $table->string('url', 255);
            $table->boolean('isActive')->default('true');
            $table->integer('order')->nullable();
            $table->integer('user');
            $table->integer('category')->nullable();

            $table->timestamps();
            $table->softDeletes();
        });

        
    }

    public function down(): void
    {
        Schema::dropIfExists('links');
    }
};
