from django.db import models

class Task(models.Model):
    completed = models.BooleanField(default=False)
    payload = models.JSONField(default=dict)
